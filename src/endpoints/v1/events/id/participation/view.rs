use utoipa::ToSchema;

use crate::database::event::get_event_members::view::Participation;

/// Answer of the caller to the invitation of the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ParticipationAnswer {
    Accepted,
    Declined,
}

impl From<ParticipationAnswer> for Participation {
    fn from(answer: ParticipationAnswer) -> Self {
        match answer {
            ParticipationAnswer::Accepted => Participation::Accepted,
            ParticipationAnswer::Declined => Participation::Declined,
        }
    }
}

/// Body of `PUT /api/v1/events/{event_id}/participation`. An unknown field is a `400`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateParticipationView {
    /// `accepted`: the caller takes part in the event; `declined`: they do not.
    pub status: ParticipationAnswer,
}
