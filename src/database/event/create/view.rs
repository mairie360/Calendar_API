use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};

#[derive(Debug, Clone, PartialEq)]
pub enum ReccurenceType {
    Daily,
    Weekly,
    Monthly,
    Error,
}

impl Display for ReccurenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ReccurenceType {
    pub fn as_str(&self) -> &str {
        match self {
            ReccurenceType::Daily => "Daily",
            ReccurenceType::Weekly => "Weekly",
            ReccurenceType::Monthly => "Monthly",
            ReccurenceType::Error => "Error",
        }
    }
}

impl From<String> for ReccurenceType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Daily" => ReccurenceType::Daily,
            "Weekly" => ReccurenceType::Weekly,
            "Monthly" => ReccurenceType::Monthly,
            _ => ReccurenceType::Error,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecurrenceRule {
    type_recurrence: ReccurenceType,
    intervalle: Option<i32>,
    date_fin: Option<chrono::DateTime<chrono::Utc>>,
}

impl RecurrenceRule {
    pub fn new(
        type_recurrence: ReccurenceType,
        intervalle: Option<i32>,
        date_fin: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self {
            type_recurrence,
            intervalle,
            date_fin,
        }
    }

    pub fn type_recurrence(&self) -> &ReccurenceType {
        &self.type_recurrence
    }

    pub fn intervalle(&self) -> Option<i32> {
        self.intervalle
    }

    pub fn date_fin(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.date_fin
    }
}

impl Display for RecurrenceRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "type_recurrence: {:}, intervalle: {:?}, date_fin: {:?}",
            self.type_recurrence, self.intervalle, self.date_fin
        )
    }
}

// La requête d'insertion est identique pour un événement créé par un utilisateur
// ou par un groupe : seule la sémantique de `owner_id` change.
const CREATE_EVENT_SQL: &str =
    "INSERT INTO events (name, description, start_date, end_date, created_by, owner_id) \
     VALUES ($1, NULLIF($2, ''), $3, $4, $5, $6) RETURNING id";

fn create_event_params(
    name: &str,
    description: Option<&str>,
    start_date: chrono::DateTime<chrono::Utc>,
    end_date: chrono::DateTime<chrono::Utc>,
    created_by: u64,
    owner_id: u64,
) -> Vec<QueryParam> {
    vec![
        QueryParam::Text(name.to_string()),
        QueryParam::Text(description.unwrap_or_default().to_string()),
        QueryParam::DateTime(start_date),
        QueryParam::DateTime(end_date),
        QueryParam::I32(created_by as i32),
        QueryParam::I32(owner_id as i32),
    ]
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateEventByUserQueryView {
    params: Vec<QueryParam>,
}

impl CreateEventByUserQueryView {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &str,
        description: Option<&str>,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        created_by: u64,
        _recurrence: Option<RecurrenceRule>,
        owner_id: u64,
    ) -> Self {
        Self {
            params: create_event_params(
                name,
                description,
                start_date,
                end_date,
                created_by,
                owner_id,
            ),
        }
    }

    pub fn name(&self) -> &str {
        self.params[0].as_text()
    }

    pub fn description(&self) -> &str {
        self.params[1].as_text()
    }

    pub fn start_date(&self) -> chrono::DateTime<chrono::Utc> {
        self.params[2].as_datetime()
    }

    pub fn end_date(&self) -> chrono::DateTime<chrono::Utc> {
        self.params[3].as_datetime()
    }

    pub fn created_by(&self) -> u64 {
        self.params[4].as_i32() as u64
    }

    pub fn owner_id(&self) -> u64 {
        self.params[5].as_i32() as u64
    }
}

impl ApiRequestDto for CreateEventByUserQueryView {
    fn query_sql(&self) -> &'static str {
        CREATE_EVENT_SQL
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

impl Display for CreateEventByUserQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, description: {:?}, start_date: {}, end_date: {}, created_by: {}, owner_id: {}",
            self.name(),
            self.description(),
            self.start_date(),
            self.end_date(),
            self.created_by(),
            self.owner_id()
        )
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateEventByGroupQueryView {
    params: Vec<QueryParam>,
}

impl CreateEventByGroupQueryView {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        description: Option<String>,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        created_by: u64,
        _recurrence: Option<RecurrenceRule>,
        owner_id: u64,
    ) -> Self {
        Self {
            params: create_event_params(
                &name,
                description.as_deref(),
                start_date,
                end_date,
                created_by,
                owner_id,
            ),
        }
    }

    pub fn name(&self) -> &str {
        self.params[0].as_text()
    }

    pub fn description(&self) -> &str {
        self.params[1].as_text()
    }

    pub fn start_date(&self) -> chrono::DateTime<chrono::Utc> {
        self.params[2].as_datetime()
    }

    pub fn end_date(&self) -> chrono::DateTime<chrono::Utc> {
        self.params[3].as_datetime()
    }

    pub fn created_by(&self) -> u64 {
        self.params[4].as_i32() as u64
    }

    pub fn owner_id(&self) -> u64 {
        self.params[5].as_i32() as u64
    }
}

impl ApiRequestDto for CreateEventByGroupQueryView {
    fn query_sql(&self) -> &'static str {
        CREATE_EVENT_SQL
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

impl Display for CreateEventByGroupQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, description: {:?}, start_date: {}, end_date: {}, created_by: {}, owner_group_id: {}",
            self.name(),
            self.description(),
            self.start_date(),
            self.end_date(),
            self.created_by(),
            self.owner_id()
        )
    }
}
