use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Droits de l'utilisateur `$2` sur l'événement `$1`, calculés en une requête.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAccessQueryView {
    params: Vec<QueryParam>,
}

impl EventAccessQueryView {
    pub fn new(event_id: u64, user_id: u64) -> Self {
        Self {
            params: vec![
                QueryParam::I32(event_id as i32),
                QueryParam::I32(user_id as i32),
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
            crate::event_requires_approval_sql!(),
            ", \
                'approval_status', CASE \
                    WHEN EXISTS (SELECT 1 FROM event_members WHERE event_id = $1 AND validation_status = 'refused') \
                        THEN 'rejected' \
                    WHEN EXISTS (SELECT 1 FROM event_members WHERE event_id = $1 AND validation_status = 'pending') \
                        THEN 'pending' \
                    ELSE 'approved' END \
            )"
        )
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventAccess {
    pub exists: bool,
    pub created_by: Option<i32>,
    pub is_member: bool,
    pub manager_role: bool,
    pub responsable_role: bool,
    pub shares_group_with_creator: bool,
    pub requires_approval: bool,
    pub approval_status: ApprovalStatus,
    /// Identifiant de l'appelant, renseigné par la vue (non lu en base).
    #[serde(skip)]
    pub caller_id: i32,
}

impl EventAccess {
    pub fn is_creator(&self) -> bool {
        self.created_by == Some(self.caller_id)
    }

    /// Seules les personnes assignées voient l'événement.
    pub fn can_view(&self) -> bool {
        self.is_member
    }

    /// Personne assignée qui en est le créateur ou qui est Responsable, Maire ou Admin.
    pub fn can_edit(&self) -> bool {
        self.is_member && (self.is_creator() || self.manager_role)
    }

    pub fn can_delete(&self) -> bool {
        self.is_creator()
    }

    /// Le créateur gère les membres dès la création (avant d'être lui-même membre).
    pub fn can_manage_members(&self) -> bool {
        self.is_creator() || self.can_edit()
    }

    /// Responsable assigné, partageant un groupe avec le créateur, d'un événement en attente de validation.
    pub fn can_validate(&self) -> bool {
        self.responsable_role
            && self.is_member
            && self.approval_status == ApprovalStatus::Pending
            && self.created_by.is_some()
            && self.requires_approval
            && self.shares_group_with_creator
    }
}
