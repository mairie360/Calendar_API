use std::fmt::Display;

use mairie360_api_lib::database::db_interface::DatabaseQueryView;

#[derive(Debug, Clone, PartialEq)]
pub enum ReccurenceType {
    Daily,
    Weekly,
    Monthly,
    Error,
}

impl Display for ReccurenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ReccurenceType {
    pub fn to_string(&self) -> &str {
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

pub struct CreateEventQueryView {
    name: String,
    description: Option<String>,
    start_date: chrono::DateTime<chrono::Utc>,
    end_date: chrono::DateTime<chrono::Utc>,
    created_by: u64,
    recurrence: Option<RecurrenceRule>,
}

impl CreateEventQueryView {
    pub fn new(
        name: String,
        description: Option<String>,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        created_by: u64,
        recurrence: Option<RecurrenceRule>,
    ) -> Self {
        Self {
            name,
            description,
            start_date,
            end_date,
            created_by,
            recurrence,
        }
    }
}

impl DatabaseQueryView for CreateEventQueryView {
    fn get_request(&self) -> String {
        "INSERT INTO events (name, description, start_date, end_date, created_by, recurrence) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id".to_string()
    }
}

impl Display for CreateEventQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, description: {:?}, start_date: {}, end_date: {}, created_by: {}, recurrence: {:?}", self.name, self.description, self.start_date, self.end_date, self.created_by, self.recurrence)
    }
}
