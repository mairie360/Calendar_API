use actix_web::web;
use utoipa::ToSchema;

use crate::endpoints::v1::events::id::members::post::endpoint::AddMemberError;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, ToSchema)]
pub struct PostMemberView {
    user_id: u64,
}

impl PostMemberView {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}

impl TryFrom<web::Json<PostMemberView>> for PostMemberView {
    type Error = AddMemberError;

    fn try_from(params: web::Json<PostMemberView>) -> Result<PostMemberView, Self::Error> {
        Ok(params.into_inner())
    }
}
