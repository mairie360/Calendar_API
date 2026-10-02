use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};

use crate::database::event::access::view::ApprovalStatus;

/// Records the approval decision `$3` of user `$2` on event `$1`, which must still be pending, and
/// returns the event id. No row (`DbError::NotFound`) means the event is no longer pending: another
/// Responsable decided first. `pending` clears the decision.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetEventApprovalQueryView {
    params: Vec<QueryParam>,
}

impl SetEventApprovalQueryView {
    pub fn new(event_id: u64, decided_by: u64, status: ApprovalStatus) -> Self {
        Self {
            params: vec![
                QueryParam::I32(event_id as i32),
                QueryParam::I32(decided_by as i32),
                QueryParam::Text(status.as_db().to_string()),
            ],
        }
    }
}

impl ApiRequestDto for SetEventApprovalQueryView {
    fn query_sql(&self) -> &'static str {
        "UPDATE events SET approval_status = $3::event_validation_status, \
            approval_decided_by = CASE WHEN $3 = 'pending' THEN NULL ELSE $2 END, \
            approval_decided_at = CASE WHEN $3 = 'pending' THEN NULL ELSE now() END \
         WHERE id = $1 AND approval_status = 'pending' RETURNING id"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

/// True when user `$2` (not archived) can be assigned by `$1`: Admin and Maire assign anyone, the
/// others themselves and the members of their groups.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CanAssignUserQueryView {
    params: Vec<QueryParam>,
}

impl CanAssignUserQueryView {
    pub fn new(caller_id: u64, user_id: u64) -> Self {
        Self {
            params: vec![
                QueryParam::I32(caller_id as i32),
                QueryParam::I32(user_id as i32),
            ],
        }
    }
}

impl ApiRequestDto for CanAssignUserQueryView {
    fn query_sql(&self) -> &'static str {
        "SELECT EXISTS (SELECT 1 FROM users u \
            WHERE u.id = $2 AND COALESCE(u.is_archived, false) = false \
              AND (u.id = $1 \
                OR EXISTS (SELECT 1 FROM user_roles ur JOIN roles r ON r.id = ur.role_id \
                    WHERE ur.user_id = $1 AND r.name IN ('Admin', 'Maire')) \
                OR EXISTS (SELECT 1 FROM group_members caller_group \
                    JOIN group_members user_group ON user_group.group_id = caller_group.group_id \
                    WHERE caller_group.user_id = $1 AND user_group.user_id = u.id)))"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}
