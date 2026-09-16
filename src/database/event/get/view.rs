use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};

use crate::database::event::model::{EventCategory, EventInput, EventRecurrence, EventVisibility};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetEventQueryView {
    params: Vec<QueryParam>,
}

impl GetEventQueryView {
    pub fn new(id: u64) -> Self {
        Self {
            params: vec![QueryParam::I32(id as i32)],
        }
    }

    pub fn id(&self) -> u64 {
        self.params[0].as_i32() as u64
    }
}

impl Display for GetEventQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetEventQueryView(id={})", self.id())
    }
}

impl ApiRequestDto for GetEventQueryView {
    fn query_sql(&self) -> &'static str {
        concat!(
            "SELECT to_jsonb(t) FROM ( \
                SELECT e.name, e.description, e.created_by, e.recurrence_id, e.start_date, e.end_date, \
                    e.owner_id, e.visibility, e.category, e.service_label AS service, e.location, ",
            crate::recurrence_json_sql!(),
            " AS recurrence \
                FROM events e LEFT JOIN recurrence_rules rr ON rr.id = e.recurrence_id \
                WHERE e.id = $1 \
             ) t"
        )
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GetEventQueryResultView {
    pub name: String,
    pub description: Option<String>,
    pub created_by: Option<i32>,
    pub recurrence_id: Option<i32>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub owner_id: Option<i32>,
    pub visibility: String,
    pub category: String,
    pub service: Option<String>,
    pub location: Option<String>,
    pub recurrence: Option<EventRecurrence>,
}

impl GetEventQueryResultView {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn created_by(&self) -> Option<i32> {
        self.created_by
    }

    pub fn recurrence_id(&self) -> Option<i32> {
        self.recurrence_id
    }

    pub fn start_date(&self) -> chrono::DateTime<chrono::Utc> {
        self.start_date
    }

    pub fn end_date(&self) -> chrono::DateTime<chrono::Utc> {
        self.end_date
    }

    pub fn owner_id(&self) -> Option<i32> {
        self.owner_id
    }

    pub fn visibility(&self) -> EventVisibility {
        if self.visibility == "private" {
            EventVisibility::Private
        } else {
            EventVisibility::Public
        }
    }

    pub fn category(&self) -> EventCategory {
        serde_json::from_value(serde_json::Value::String(self.category.clone())).unwrap_or_default()
    }

    /// Données de l'événement au format d'écriture, pour appliquer une modification partielle.
    pub fn to_input(&self) -> EventInput {
        EventInput {
            name: self.name.clone(),
            description: self.description.clone(),
            start: self.start_date,
            end: self.end_date,
            visibility: self.visibility(),
            category: self.category(),
            service: self.service.clone(),
            location: self.location.clone(),
            recurrence: self.recurrence.clone(),
        }
    }
}
