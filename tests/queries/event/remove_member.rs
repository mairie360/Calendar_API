use std::{thread::sleep, time::Duration};

use crate::common::get_pool;
use calendar_api::database::event::{
    add_member::{query::add_user_to_event_query, view::AddUserToEventQueryView},
    create::{query::create_event_by_user_query, view::CreateEventByUserQueryView},
    get::{query::get_event_query, view::GetEventQueryView},
    remove_member::{query::remove_user_from_event_query, view::RemoveUserFromEventQueryView},
};
use chrono::Utc;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;

#[sqlx::test]
async fn test_remove_member_success() {
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

    let view = AddUserToEventQueryView::new(2, id);
    let result = add_user_to_event_query(view, pool.clone()).await;
    assert!(result.is_ok());

    let view = RemoveUserFromEventQueryView::new(2, id);
    let result = remove_user_from_event_query(view, pool.clone()).await;
    assert!(result.is_ok());

    // On suppose que l'ID 1 a été créé par le test de création ou le setup
    let view = GetEventQueryView::new(id);

    let result = get_event_query(view, pool).await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[sqlx::test]
async fn test_remove_member_event_not_found() {
    let (_container, host) = get_shared_db().await;
    let pool = get_pool(host.to_string()).await;

    let view = RemoveUserFromEventQueryView::new(1, 99999);
    let result = remove_user_from_event_query(view, pool.clone()).await;
    assert!(result.is_ok());
}

#[sqlx::test]
async fn test_remove_member_user_not_found() {
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

    let view = RemoveUserFromEventQueryView::new(999, id);
    let result = remove_user_from_event_query(view, pool.clone()).await;
    assert!(result.is_ok());
}

#[sqlx::test]
async fn test_remove_member_user_and_event_not_found() {
    let (_container, host) = get_shared_db().await;
    let pool = get_pool(host.to_string()).await;

    let view = RemoveUserFromEventQueryView::new(999, 99999);
    let result = remove_user_from_event_query(view, pool.clone()).await;
    assert!(result.is_ok());
}
