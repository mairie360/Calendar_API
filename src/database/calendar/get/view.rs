use std::fmt::Display;

use mairie360_api_lib::database::db_interface::DatabaseQueryView;

pub struct GetCalendarQueryView {
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
}

impl GetCalendarQueryView {
    pub fn new(start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.start
    }

    pub fn end(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.end
    }
}

impl Display for GetCalendarQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GetCalendarQueryView: start={} end={}",
            self.start, self.end
        )
    }
}

impl DatabaseQueryView for GetCalendarQueryView {
    fn get_request(&self) -> String {
        "SELECT id, title, start_date, end_date FROM events WHERE start >= $1 AND end <= $2"
            .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    id: i64,
    title: String,
    start_date: chrono::DateTime<chrono::Utc>,
    end_date: chrono::DateTime<chrono::Utc>,
    recurrence_id: Option<i64>,
}

impl Event {
    pub fn new(
        id: i64,
        title: String,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        recurrence_id: Option<i64>,
    ) -> Self {
        Self {
            id,
            title,
            start_date,
            end_date,
            recurrence_id,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn start_date(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.start_date
    }

    pub fn end_date(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.end_date
    }

    pub fn recurrence_id(&self) -> Option<i64> {
        self.recurrence_id
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Event: id={} title={} start={} end={} recurrence_id={:?}",
            self.id, self.title, self.start_date, self.end_date, self.recurrence_id
        )
    }
}

pub struct GetCalendarQueryResultView {
    events: Vec<Event>,
}

impl GetCalendarQueryResultView {
    pub fn new(events: Vec<Event>) -> Self {
        Self { events }
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }
}

impl Display for GetCalendarQueryResultView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetCalendarQueryResultView: events={:?}", self.events)
    }
}
