use crate::database::event::create::view::{
    CreateEventByGroupQueryView, CreateEventByUserQueryView,
};
use mairie360_api_lib::database::db_interface::DatabaseQueryView;
use mairie360_api_lib::database::errors::DatabaseError;
use sqlx::PgPool;

pub async fn create_event_by_user_query(
    view: CreateEventByUserQueryView,
    pool: PgPool,
) -> Result<i32, DatabaseError> {
    // Utilisation de query_scalar car on s'attend à recevoir une seule valeur : le "RETURNING id"
    let inserted_id = sqlx::query_scalar::<_, i32>(&view.get_request())
        .bind(view.name()) // Nécessite de créer les getters correspondants dans ta vue
        .bind(view.description())
        .bind(view.start_date())
        .bind(view.end_date())
        .bind(view.created_by() as i32)
        .bind(view.owner_id() as i32)
        .fetch_one(&pool)
        .await?;

    Ok(inserted_id)
}

pub async fn create_event_by_group_query(
    view: CreateEventByGroupQueryView,
    pool: PgPool,
) -> Result<i32, DatabaseError> {
    // Utilisation de query_scalar car on s'attend à recevoir une seule valeur : le "RETURNING id"
    let inserted_id = sqlx::query_scalar::<_, i32>(&view.get_request())
        .bind(view.name()) // Nécessite de créer les getters correspondants dans ta vue
        .bind(view.description())
        .bind(view.start_date())
        .bind(view.end_date())
        .bind(view.created_by() as i32)
        .bind(view.owner_id() as i32)
        .fetch_one(&pool)
        .await?;

    Ok(inserted_id)
}
