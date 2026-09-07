use chrono::{DateTime, Utc};
use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EditEventQueryView {
    params: Vec<QueryParam>,
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
            params: vec![
                QueryParam::Text(title),
                QueryParam::Text(description.unwrap_or_default()),
                QueryParam::DateTime(start_date),
                QueryParam::DateTime(end_date),
                QueryParam::Text(location.unwrap_or_default()),
                QueryParam::I32(id as i32),
            ],
        }
    }

    pub fn id(&self) -> u64 {
        self.params[5].as_i32() as u64
    }
    pub fn title(&self) -> &str {
        self.params[0].as_text()
    }
    pub fn description(&self) -> &str {
        self.params[1].as_text()
    }
    pub fn start_date(&self) -> DateTime<Utc> {
        self.params[2].as_datetime()
    }
    pub fn end_date(&self) -> DateTime<Utc> {
        self.params[3].as_datetime()
    }
    pub fn location(&self) -> &str {
        self.params[4].as_text()
    }
}

impl ApiRequestDto for EditEventQueryView {
    fn query_sql(&self) -> &'static str {
        // L'id est positionné en dernier ($6) dans la clause WHERE.
        "UPDATE events
         SET title = $1, description = NULLIF($2, ''), start_date = $3, end_date = $4, \
             location = NULLIF($5, ''), updated_at = NOW()
         WHERE id = $6"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}
