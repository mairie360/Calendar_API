use crate::database::event::get::view::{GetEventQueryResultView, GetEventQueryView};
use mairie360_api_lib::database::db_interface::DatabaseQueryView;
use mairie360_api_lib::database::errors::DatabaseError;
use sqlx::PgPool;

pub async fn get_event_query(
    view: GetEventQueryView,
    pool: PgPool, // ou &PgPool selon ta gestion d'état
) -> Result<Option<GetEventQueryResultView>, DatabaseError> {
    let result = sqlx::query_as::<_, GetEventQueryResultView>(&view.get_request())
        .bind(view.id() as i32) // cast éventuel si l'ID en base (SERIAL) est un i32
        .fetch_optional(&pool)
        .await?;

    Ok(result)
}
