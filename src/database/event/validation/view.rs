use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};

use crate::database::event::get_event_members::view::EventValidationStatus;

/// Applique un statut de validation à tous les membres de l'événement.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetEventValidationQueryView {
    params: Vec<QueryParam>,
}

impl SetEventValidationQueryView {
    pub fn new(event_id: u64, status: EventValidationStatus) -> Self {
        Self {
            params: vec![
                QueryParam::I32(event_id as i32),
                QueryParam::Text(status.to_string()),
            ],
        }
    }
}

impl ApiRequestDto for SetEventValidationQueryView {
    fn query_sql(&self) -> &'static str {
        "UPDATE event_members SET validation_status = $2::event_validation_status WHERE event_id = $1"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

/// Recalcule la validation après un changement de membres : « en attente » si l'événement doit être validé
/// par un responsable, « validé » sinon. Sans créateur connu, les statuts sont conservés.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RefreshEventValidationQueryView {
    params: Vec<QueryParam>,
}

impl RefreshEventValidationQueryView {
    pub fn new(event_id: u64) -> Self {
        Self {
            params: vec![QueryParam::I32(event_id as i32)],
        }
    }
}

impl ApiRequestDto for RefreshEventValidationQueryView {
    fn query_sql(&self) -> &'static str {
        concat!(
            "UPDATE event_members SET validation_status = (CASE WHEN ",
            crate::event_requires_approval_sql!(),
            " THEN 'pending' ELSE 'validated' END)::event_validation_status \
             WHERE event_id = $1 \
               AND EXISTS (SELECT 1 FROM events WHERE id = $1 AND created_by IS NOT NULL)"
        )
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

/// Vrai si l'utilisateur `$2` (non archivé) peut être assigné par `$1` : Admin et Maire assignent tout le
/// monde, les autres eux-mêmes et les membres de leurs groupes.
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
