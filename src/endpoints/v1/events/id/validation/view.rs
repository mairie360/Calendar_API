use utoipa::ToSchema;

use crate::database::event::access::view::ApprovalStatus;

/// Approval decision to record on the event. An unknown field is a `400`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateEventValidationView {
    /// `approved` approves the event, `rejected` refuses it, `pending` leaves it waiting.
    pub status: ApprovalStatus,
}
