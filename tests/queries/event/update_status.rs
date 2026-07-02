use std::{thread::sleep, time::Duration};

use crate::common::get_pool;
use calendar_api::database::event::{
    create::{query::create_event_by_user_query, view::CreateEventByUserQueryView},
    delete::{query::delete_event_query, view::DeleteEventQueryView},
    get::{query::get_event_query, view::GetEventQueryView},
};
use chrono::Utc;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;

#[sqlx::test]
async fn test_delete_event_success() {
    let (_container, host) = get_shared_db().await;
    let pool = get_pool(host.to_string()).await;
    let start = Utc::now();
    sleep(Duration::from_secs(1));
    let end = Utc::now();

    let view = CreateEventByUserQueryView::new(
        "Test Event",
        Some("Description"),
        start,
        end,
        1, // ID utilisateur admin (assure-toi qu'il existe en DB)
        None,
        1,
    );

    let result = create_event_by_user_query(view, pool.clone()).await;
    let id = result.unwrap() as u64;

    let view = DeleteEventQueryView::new(id);
    let result = delete_event_query(view, pool.clone()).await;
    assert!(result.is_ok());
}

#[sqlx::test]
async fn test_get_delete_not_found() {
    let (_container, host) = get_shared_db().await;
    let pool = get_pool(host.to_string()).await;

    let view = DeleteEventQueryView::new(99999);
    let result = delete_event_query(view, pool.clone()).await;
    assert!(result.is_ok());

    // ID qui n'existe certainement pas en base
    let view = GetEventQueryView::new(99999);

    let result = get_event_query(view, pool).await;

    // Le résultat doit être Ok, mais avec None (pas une erreur de DB)
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}
