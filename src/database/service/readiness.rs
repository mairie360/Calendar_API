use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};
use mairie360_api_lib::state::AppState;

/// `SELECT 1`: answers as soon as Postgres accepts queries on the pool.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PingDatabaseQueryView {
    params: Vec<QueryParam>,
}

impl ApiRequestDto for PingDatabaseQueryView {
    fn query_sql(&self) -> &'static str {
        "SELECT 1"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

/// A dependency the API cannot serve requests without.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dependency {
    Postgres,
    Redis,
}

impl std::fmt::Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Dependency::Postgres => "PostgreSQL",
            Dependency::Redis => "Redis",
        })
    }
}

/// Key read to probe Redis; it never exists, a reachable server answers `nil`.
const REDIS_PROBE_KEY: &str = "readiness-probe";

/// Queries Postgres, then Redis (the revocation list of session tokens lives there, so an API
/// without it refuses every session token). Returns the first unreachable dependency.
pub async fn check_dependencies(state: &AppState) -> Result<(), Dependency> {
    let db = state.get_smart_db();
    if let Err(error) = db
        .fetch_scalar::<i32, _>(&PingDatabaseQueryView::default())
        .await
    {
        log::warn!(target: "calendar_api::readiness", "PostgreSQL unreachable: {error:?}");
        return Err(Dependency::Postgres);
    }
    if let Err(error) = state
        .get_redis()
        .secure_get::<String>(REDIS_PROBE_KEY)
        .await
    {
        log::warn!(target: "calendar_api::readiness", "Redis unreachable: {error:?}");
        return Err(Dependency::Redis);
    }
    Ok(())
}
