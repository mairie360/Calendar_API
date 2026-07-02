use crate::database::event::edit::view::EditEventQueryView;
use mairie360_api_lib::database::db_interface::DatabaseQueryView;
use mairie360_api_lib::database::errors::DatabaseError;
use sqlx::PgPool;

pub async fn edit_event_query(
    view: EditEventQueryView,
    pool: PgPool,
) -> Result<u64, DatabaseError> {
    let result = sqlx::query(&view.get_request())
        .bind(view.title())
        .bind(view.description())
        .bind(view.start_date())
        .bind(view.end_date())
        .bind(view.location())
        .bind(view.id() as i32)
        .execute(&pool)
        .await?;

    Ok(result.rows_affected()) // Vérifie si l'événement existait et a bien été modifié
}
