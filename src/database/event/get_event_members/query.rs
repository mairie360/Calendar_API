use crate::database::event::get_event_members::view::{GetEventMemberQueryView, Member};
use mairie360_api_lib::database::db_interface::DatabaseQueryView;
use mairie360_api_lib::database::errors::DatabaseError;
use sqlx::PgPool;

pub async fn get_event_members_query(
    view: GetEventMemberQueryView,
    pool: PgPool,
) -> Result<Vec<Member>, DatabaseError> {
    let result = sqlx::query_as::<_, Member>(&view.get_request())
        .bind(view.event_id() as i32)
        .fetch_all(&pool)
        .await?;

    Ok(result)
}
