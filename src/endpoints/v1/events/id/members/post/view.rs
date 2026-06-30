use actix_web::web;
use utoipa::ToSchema;

use crate::endpoints::v1::events::id::{
    get::view::Member, members::post::endpoint::AddMemberError,
};

#[derive(Debug, Clone, PartialEq, serde::Deserialize, ToSchema)]
pub struct PostMemberView {
    member: Member,
}

impl PostMemberView {
    pub fn new(member: Member) -> Self {
        Self { member }
    }

    pub fn member(&self) -> &Member {
        &self.member
    }
}

impl TryFrom<web::Json<PostMemberView>> for PostMemberView {
    type Error = AddMemberError;

    fn try_from(params: web::Json<PostMemberView>) -> Result<PostMemberView, Self::Error> {
        Ok(params.into_inner())
    }
}
