use std::{thread::sleep, time::Duration};

use crate::common::get_pool;
use calendar_api::database::event::{
    create::{query::create_event_by_user_query, view::CreateEventByUserQueryView},
    get::{query::get_event_query, view::GetEventQueryView},
};
use chrono::Utc;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;

#[sqlx::test]
async fn test_get_event_success() {
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

    // On suppose que l'ID 1 a été créé par le test de création ou le setup
    let view = GetEventQueryView::new(result.unwrap() as u64);

    let result = get_event_query(view, pool).await;

    assert!(result.is_ok());
    let event = result.unwrap();
    assert!(event.is_some());
    let event = event.unwrap();
    //test les différentes champs
    assert_eq!(event.name(), "Test Event");
    assert_eq!(event.description(), Some("Description"));
    assert_eq!(event.created_by(), Some(1));
    assert_eq!(event.recurrence_id(), None);
    assert_eq!(
        event.start_date().timestamp_millis(),
        start.timestamp_millis()
    );
    assert_eq!(event.end_date().timestamp_millis(), end.timestamp_millis());
}

#[sqlx::test]
async fn test_get_event_not_found() {
    let (_container, host) = get_shared_db().await;
    let pool = get_pool(host.to_string()).await;

    // ID qui n'existe certainement pas en base
    let view = GetEventQueryView::new(99999);

    let result = get_event_query(view, pool).await;

    // Le résultat doit être Ok, mais avec None (pas une erreur de DB)
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}
