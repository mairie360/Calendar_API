use std::fmt::Display;

use mairie360_api_lib::database::db_interface::DatabaseQueryView;

pub struct GetEventQueryView {
    id: u64,
}

impl GetEventQueryView {
    pub fn new(id: u64) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

impl Display for GetEventQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetEventQueryView(id={})", self.id)
    }
}

impl DatabaseQueryView for GetEventQueryView {
    fn get_request(&self) -> String {
        "SELECT name, description, created_by, recurrence_id, start_date, end_date, owner_id from events WHERE id = $1".to_string()
    }
}

#[derive(Debug, sqlx::FromRow, PartialEq, Eq)]
pub struct GetEventQueryResultView {
    name: String,
    description: Option<String>,
    created_by: Option<i32>,
    recurrence_id: Option<i32>,
    start_date: chrono::DateTime<chrono::Utc>,
    end_date: chrono::DateTime<chrono::Utc>,
    owner_id: Option<i32>,
}

impl GetEventQueryResultView {
    pub fn new(
        name: String,
        description: Option<String>,
        created_by: Option<i32>,
        recurrence_id: Option<i32>,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        owner_id: Option<i32>,
    ) -> Self {
        Self {
            name,
            description,
            created_by,
            recurrence_id,
            start_date,
            end_date,
            owner_id,
        }
    }

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
}

impl Display for GetEventQueryResultView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GetEventQueryResultView: {{name: {:?}, description: {:?}, created_by: {:?}, recurrence_id: {:?}, start_date: {:?}, end_date: {:?}, owner_id: {:?}}}",
            self.name,
            self.description,
            self.created_by,
            self.recurrence_id,
            self.start_date,
            self.end_date,
            self.owner_id,
        )
    }
}
