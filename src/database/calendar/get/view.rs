use std::fmt::Display;

use mairie360_api_lib::database::db_interface::DatabaseQueryView;

pub struct GetCalendarQueryView {
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
    user_id: u64,
}

impl GetCalendarQueryView {
    pub fn new(
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
        user_id: u64,
    ) -> Self {
        Self {
            start,
            end,
            user_id,
        }
    }

    pub fn start(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.start
    }

    pub fn end(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.end
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}

impl Display for GetCalendarQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GetCalendarQueryView: start={} end={} user_id={}",
            self.start, self.end, self.user_id
        )
    }
}

impl DatabaseQueryView for GetCalendarQueryView {
    fn get_request(&self) -> String {
        // On utilise DISTINCT pour éviter les doublons si un user est à la fois
        // propriétaire et membre inscrit.
        "SELECT DISTINCT e.id, e.name, e.start_date, e.end_date
         FROM events e
         LEFT JOIN event_members em ON e.id = em.event_id
         WHERE (em.user_id = $3 OR e.owner_id = $3)
         AND e.start_date >= $1
         AND e.end_date <= $2"
            .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Event {
    id: i32,
    name: String,
    start_date: chrono::DateTime<chrono::Utc>,
    end_date: chrono::DateTime<chrono::Utc>,
}

impl Event {
    pub fn new(
        id: i32,
        name: &str,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            start_date,
            end_date,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn start_date(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.start_date
    }

    pub fn end_date(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.end_date
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Event: id={} name={} start={} end={}",
            self.id, self.name, self.start_date, self.end_date
        )
    }
}
