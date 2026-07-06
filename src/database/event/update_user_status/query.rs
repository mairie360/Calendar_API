use crate::database::event::update_user_status::view::EventStatusUpdateQueryView;
use mairie360_api_lib::database::db_interface::DatabaseQueryView;
use mairie360_api_lib::database::errors::DatabaseError;
use sqlx::PgPool;

pub async fn accept_event_query(
    view: EventStatusUpdateQueryView,
    pool: PgPool,
) -> Result<u64, DatabaseError> {
    let result = sqlx::query(&view.get_request())
        .bind(view.status().to_string())
        .bind(view.event_id() as i32)
        .bind(view.user_id() as i32)
        .execute(&pool)
        .await?;

    // Retourne le nombre de lignes modifiées (devrait être 1)
    Ok(result.rows_affected())
}
