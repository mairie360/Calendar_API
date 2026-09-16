use std::sync::atomic::{AtomicU64, Ordering};

use calendar_api::database::event::create::view::CreateEventQueryView;
use calendar_api::database::event::model::{EventCategory, EventInput, EventVisibility};
use chrono::{DateTime, Utc};
use mairie360_api_lib::database::db_interface::{ApiRequestDto, Database, QueryParam};

/// Données minimales d'un événement public, sans métadonnées ni répétition.
pub fn event_input(
    name: &str,
    description: Option<&str>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> EventInput {
    EventInput {
        name: name.to_string(),
        description: description.map(str::to_string),
        start,
        end,
        visibility: EventVisibility::Public,
        category: EventCategory::Other,
        service: None,
        location: None,
        recurrence: None,
    }
}

pub async fn create_event(db: &Database, creator_id: u64, input: &EventInput) -> u64 {
    db.fetch_scalar::<i32, _>(&CreateEventQueryView::new(creator_id, input))
        .await
        .unwrap() as u64
}

/// Requête SQL de mise en place propre aux tests (utilisateurs, rôles, groupes), sans paramètres.
#[derive(serde::Serialize, serde::Deserialize)]
struct FixtureSql {
    #[serde(skip)]
    sql: &'static str,
    params: Vec<QueryParam>,
}

impl ApiRequestDto for FixtureSql {
    fn query_sql(&self) -> &'static str {
        self.sql
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

fn fixture(sql: String) -> FixtureSql {
    FixtureSql {
        sql: Box::leak(sql.into_boxed_str()),
        params: Vec::new(),
    }
}

fn unique_suffix() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{nanos}{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

/// Crée un utilisateur (avec un rôle si `role` est fourni) et renvoie son identifiant.
pub async fn create_user(db: &Database, first_name: &str, role: Option<&str>) -> u64 {
    let id: i32 = db
        .fetch_scalar(&fixture(format!(
            "INSERT INTO users (first_name, last_name, email, password) \
             VALUES ('{first_name}', 'Test', '{}.{}@calendar-api.test', 'password') RETURNING id",
            first_name.to_lowercase(),
            unique_suffix()
        )))
        .await
        .unwrap();
    if let Some(role) = role {
        db.execute(&fixture(format!(
            "INSERT INTO user_roles (user_id, role_id) SELECT {id}, id FROM roles WHERE name = '{role}'"
        )))
        .await
        .unwrap();
    }
    id as u64
}

/// Crée un groupe contenant `members`.
pub async fn create_group(db: &Database, owner_id: u64, members: &[u64]) {
    let id: i32 = db
        .fetch_scalar(&fixture(format!(
            "INSERT INTO groups (owner_id, name) VALUES ({owner_id}, 'Groupe {}') RETURNING id",
            unique_suffix()
        )))
        .await
        .unwrap();
    for member in members {
        db.execute(&fixture(format!(
            "INSERT INTO group_members (group_id, user_id) VALUES ({id}, {member}) ON CONFLICT DO NOTHING"
        )))
        .await
        .unwrap();
    }
}

/// Nombre de règles de répétition portant l'identifiant donné (0 ou 1).
pub async fn recurrence_rule_count(db: &Database, rule_id: i32) -> i64 {
    db.fetch_scalar(&fixture(format!(
        "SELECT COUNT(*) FROM recurrence_rules WHERE id = {rule_id}"
    )))
    .await
    .unwrap()
}
