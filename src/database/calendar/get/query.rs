use crate::database::calendar::get::view::Event;
use crate::database::calendar::get::view::GetCalendarQueryView;
use mairie360_api_lib::database::db_interface::DatabaseQueryView;
use mairie360_api_lib::database::errors::DatabaseError;
use sqlx::PgPool;

pub async fn get_calendar_query(
    view: GetCalendarQueryView,
    pool: PgPool, // ou &PgPool selon ta configuration globale
) -> Result<Vec<Event>, DatabaseError> {
    // 1. On exécute la requête configurée dans ta view
    let events = sqlx::query_as::<_, Event>(&view.get_request())
        .bind(view.start())
        .bind(view.end())
        .bind(view.user_id() as i32)
        .fetch_all(&pool)
        .await?;

    Ok(events)
}
