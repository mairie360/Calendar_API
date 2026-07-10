use crate::common::get_pool; // Utilisation de ta fonction utilitaire existante
use calendar_api::database::event::create::{
    query::{create_event_by_group_query, create_event_by_user_query},
    view::{
        CreateEventByGroupQueryView, CreateEventByUserQueryView, ReccurenceType, RecurrenceRule,
    },
};
use chrono::Utc;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;

#[sqlx::test]
async fn test_create_event_by_user_success() {
    let (_container, host) = get_shared_db().await;
    let pool = get_pool(host.to_string()).await;

    let view = CreateEventByUserQueryView::new(
        "Test Event",
        Some("Description"),
        Utc::now(),
        Utc::now(),
        1, // ID utilisateur admin (assure-toi qu'il existe en DB)
        None,
        1,
    );

    let result = create_event_by_user_query(view, pool).await;

    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}

#[sqlx::test]
async fn test_create_recurrent_event_by_user_success() {
    let (_container, host) = get_shared_db().await;
    let pool = get_pool(host.to_string()).await;

    let view = CreateEventByUserQueryView::new(
        "Test Event",
        Some("Description"),
        Utc::now(),
        Utc::now(),
        1, // ID utilisateur admin (assure-toi qu'il existe en DB)
        Some(RecurrenceRule::new(ReccurenceType::Daily, Some(1), None)),
        1,
    );

    let result = create_event_by_user_query(view, pool).await;

    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}

#[sqlx::test]
async fn test_create_event_by_group_success() {
    let (_container, host) = get_shared_db().await;
    let pool = get_pool(host.to_string()).await;

    let view = CreateEventByGroupQueryView::new(
        "Test Event".to_string(),
        Some("Description".to_string()),
        Utc::now(),
        Utc::now(),
        1, // ID utilisateur admin (assure-toi qu'il existe en DB)
        None,
        1,
    );

    let result = create_event_by_group_query(view, pool).await;

    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}

#[sqlx::test]
async fn test_create_recurrent_event_by_groupcreate_event_by_group_queryss() {
    let (_container, host) = get_shared_db().await;
    let pool = get_pool(host.to_string()).await;

    let view = CreateEventByGroupQueryView::new(
        "Test Event".to_string(),
        Some("Description".to_string()),
        Utc::now(),
        Utc::now(),
        1, // ID utilisateur admin (assure-toi qu'il existe en DB)
        Some(RecurrenceRule::new(ReccurenceType::Daily, Some(1), None)),
        1,
    );

    let result = create_event_by_group_query(view, pool).await;

    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}
