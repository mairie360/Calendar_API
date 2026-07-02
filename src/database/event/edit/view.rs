use chrono::{DateTime, Utc};
use mairie360_api_lib::database::db_interface::DatabaseQueryView;

#[derive(Debug, Clone)]
pub struct EditEventQueryView {
    id: u64,
    title: String,
    description: Option<String>,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    location: Option<String>,
}

impl EditEventQueryView {
    pub fn new(
        id: u64,
        title: String,
        description: Option<String>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        location: Option<String>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            start_date,
            end_date,
            location,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub fn start_date(&self) -> DateTime<Utc> {
        self.start_date
    }
    pub fn end_date(&self) -> DateTime<Utc> {
        self.end_date
    }
    pub fn location(&self) -> Option<&String> {
        self.location.as_ref()
    }
}

impl DatabaseQueryView for EditEventQueryView {
    fn get_request(&self) -> String {
        // L'id est positionné en dernier ($6) dans la clause WHERE
        "UPDATE events
         SET title = $1, description = $2, start_date = $3, end_date = $4, location = $5, updated_at = NOW()
         WHERE id = $6".to_string()
    }
}
