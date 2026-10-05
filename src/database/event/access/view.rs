use mairie360_api_lib::database::db_interface::{id_to_sql, ApiRequestDto, QueryParam};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Rights of user `$2` on event `$1`, computed in one query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAccessQueryView {
    params: Vec<QueryParam>,
}

impl EventAccessQueryView {
    pub fn new(event_id: u64, user_id: u64) -> Self {
        Self {
            params: vec![
                QueryParam::I32(id_to_sql(event_id)),
                QueryParam::I32(id_to_sql(user_id)),
            ],
        }
    }
}

impl ApiRequestDto for EventAccessQueryView {
    fn query_sql(&self) -> &'static str {
        concat!(
            "SELECT jsonb_build_object( \
                'exists', EXISTS (SELECT 1 FROM events WHERE id = $1), \
                'created_by', (SELECT created_by FROM events WHERE id = $1), \
                'is_public', EXISTS (SELECT 1 FROM events WHERE id = $1 AND visibility = 'public'), \
                'is_member', EXISTS (SELECT 1 FROM event_members WHERE event_id = $1 AND user_id = $2), \
                'manager_role', EXISTS (SELECT 1 FROM user_roles ur JOIN roles r ON r.id = ur.role_id \
                    WHERE ur.user_id = $2 AND r.name IN ('Admin', 'Maire', 'Responsable')), \
                'responsable_role', EXISTS (SELECT 1 FROM user_roles ur JOIN roles r ON r.id = ur.role_id \
                    WHERE ur.user_id = $2 AND r.name = 'Responsable'), \
                'shares_group_with_creator', EXISTS (SELECT 1 FROM events ev \
                    JOIN group_members creator_group ON creator_group.user_id = ev.created_by \
                    JOIN group_members caller_group ON caller_group.group_id = creator_group.group_id \
                        AND caller_group.user_id = $2 \
                    WHERE ev.id = $1), \
                'requires_approval', ",
            crate::creator_requires_approval_sql!("(SELECT created_by FROM events WHERE id = $1)"),
            ", \
                'approval_status', COALESCE((SELECT CASE approval_status \
                    WHEN 'refused' THEN 'rejected' WHEN 'pending' THEN 'pending' ELSE 'approved' END \
                    FROM events WHERE id = $1), 'pending') \
            )"
        )
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

/// Approval decision of an event: `pending` (waiting for a Responsable), `approved` or `rejected`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

impl ApprovalStatus {
    /// Matching `event_validation_status` value of `events.approval_status`.
    pub fn as_db(&self) -> &'static str {
        match self {
            ApprovalStatus::Pending => "pending",
            ApprovalStatus::Approved => "validated",
            ApprovalStatus::Rejected => "refused",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventAccess {
    pub exists: bool,
    pub created_by: Option<i32>,
    pub is_public: bool,
    pub is_member: bool,
    pub manager_role: bool,
    pub responsable_role: bool,
    pub shares_group_with_creator: bool,
    /// The creator only has the User or Guest roles: a Responsable must approve the event.
    pub requires_approval: bool,
    pub approval_status: ApprovalStatus,
    /// Caller id, set by the view (not read from the database).
    #[serde(skip)]
    pub caller_id: i32,
}

impl EventAccess {
    pub fn is_creator(&self) -> bool {
        self.created_by == Some(self.caller_id)
    }

    /// A public event is readable by every authenticated user; a private one by its members and
    /// its creator.
    pub fn can_view(&self) -> bool {
        self.is_public || self.is_member || self.is_creator()
    }

    /// Assigned member who is the creator or has the Responsable, Maire or Admin role.
    pub fn can_edit(&self) -> bool {
        self.is_member && (self.is_creator() || self.manager_role)
    }

    pub fn can_delete(&self) -> bool {
        self.is_creator()
    }

    /// The creator manages members from creation on (before being a member themselves).
    pub fn can_manage_members(&self) -> bool {
        self.is_creator() || self.can_edit()
    }

    /// Assigned Responsable sharing a group with the creator, on an event still waiting for
    /// approval.
    pub fn can_validate(&self) -> bool {
        self.responsable_role
            && self.is_member
            && self.approval_status == ApprovalStatus::Pending
            && self.created_by.is_some()
            && self.requires_approval
            && self.shares_group_with_creator
    }

    /// An edit moving the event (see `EventInput::changes_schedule`) made without a manager role
    /// must be approved again.
    pub fn edit_needs_new_approval(&self) -> bool {
        self.requires_approval && !self.manager_role
    }
}

/// Locks the row of event `$1` until the end of the transaction (`FOR UPDATE`), so that the
/// rights computed by `EventAccessQueryView` in the same transaction still hold when the write
/// runs: a concurrent edit, deletion, member change or decision on the same event waits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockEventQueryView {
    params: Vec<QueryParam>,
}

impl LockEventQueryView {
    pub fn new(event_id: u64) -> Self {
        Self {
            params: vec![QueryParam::I32(id_to_sql(event_id))],
        }
    }
}

impl ApiRequestDto for LockEventQueryView {
    fn query_sql(&self) -> &'static str {
        "SELECT id FROM events WHERE id = $1 FOR UPDATE"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}
