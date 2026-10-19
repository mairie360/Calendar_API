//! The code under test connects as `calendar_api`, never as the superuser (MAIR-556): a
//! superuser ignores the grants of Database's `api_grants.sql`, so a query Calendar may not run
//! in staging would pass here.

use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};
use serial_test::serial;

use crate::common::{api_db, CALENDAR_DB_ROLE};

#[derive(serde::Serialize, serde::Deserialize)]
struct RawSql {
    #[serde(skip)]
    sql: &'static str,
    params: Vec<QueryParam>,
}

impl ApiRequestDto for RawSql {
    fn query_sql(&self) -> &'static str {
        self.sql
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

fn sql(sql: &'static str) -> RawSql {
    RawSql {
        sql,
        params: Vec::new(),
    }
}

#[tokio::test]
#[serial]
async fn mair_556_the_code_under_test_runs_as_calendar_api() {
    let db = api_db().await;

    let user: String = db
        .fetch_scalar(&sql("SELECT current_user::TEXT"))
        .await
        .unwrap();
    let is_superuser: bool = db
        .fetch_scalar(&sql(
            "SELECT rolsuper FROM pg_roles WHERE rolname = current_user",
        ))
        .await
        .unwrap();

    assert_eq!(user, CALENDAR_DB_ROLE);
    assert!(!is_superuser);
}

#[tokio::test]
#[serial]
async fn mair_556_calendar_api_is_held_to_its_grants() {
    // Calendar never reads the project domain: the grants refuse it, as in staging.
    let db = api_db().await;

    let error = db
        .fetch_scalar::<String, _>(&sql("SELECT title FROM projects LIMIT 1"))
        .await
        .expect_err("calendar_api must not read projects");

    assert!(
        error.to_string().contains("SQLSTATE 42501"),
        "expected permission denied (42501), got {error}"
    );
}
