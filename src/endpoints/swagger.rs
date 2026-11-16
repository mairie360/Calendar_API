use crate::endpoints::health::HealthDoc;
use crate::endpoints::ready::ReadyDoc;
use crate::endpoints::v1::doc::V1Doc;
use mairie360_api_lib::env_manager::get_env_var;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Calendar API — Mairie 360",
        version = "1.0.0",
        description = "\
Calendar API of the **Mairie 360** platform: events, participants, recurrences and approval \
workflow. Accounts, roles and groups live in Core API.

## Two ways to read the calendar

`GET /api/v1/calendar` returns the **calendar** view: the caller's events over a given \
period, recurrences expanded. `GET /api/v1/events/{event_id}/` returns the **detail** of an \
event, with its participants, its approval status and the caller's rights on it.

## Access rights

An event is only visible to its **participants**: an event the caller is not assigned to \
answers `403`, not `404`, which is kept for ids that do not exist.

| Action | Who can do it |
| --- | --- |
| View | any assigned participant |
| Edit | a participant who created it, or who has the Responsable, Maire or Admin role |
| Delete | the creator, and only them |
| Manage the participants | the creator, or anyone who can edit the event |
| Approve | an assigned Responsable sharing a group with the creator, on a pending event |

The `permissions` field of `GET /api/v1/events/{event_id}/` gives these rights for the \
caller directly, so clients do not have to derive them.

## Error format

Error responses (`4xx` and `5xx`) have a **`text/plain`** body holding the error message, not a \
JSON object. Every response carries `X-Content-Type-Options: nosniff`.

Statuses returned across the API, before the handler runs:

| Status | Meaning |
| --- | --- |
| `400` | URL segment that is not an integer, missing or malformed query parameter, or malformed JSON body. |
| `401` | `Authorization` header missing or malformed, invalid or expired JWT, or revoked session. |
| `429` | The caller exceeded their request quota (per user, see `RATE_LIMIT_PER_SECOND` / `RATE_LIMIT_BURST`); `Retry-After` gives the seconds to wait. |
| `500` | Database or Redis failure. |
",
        contact(
            name = "Mairie 360 team",
            url = "https://github.com/mairie360"
        ),
        license(
            name = "Proprietary",
            identifier = "LicenseRef-mairie360-proprietary"
        )
    ),
    servers(
        (url = "http://localhost:3002", description = "Local development (cargo run)"),
        (url = "http://development.mairie360.fr", description = "Development Docker stack (nginx)")
    ),
    tags(
        (name = "Calendar", description = "Calendar view: the caller's events over a period, recurrences expanded."),
        (name = "Events", description = "Events: creation, detail, update, deletion, participants and approval."),
        (name = "Service", description = "Unauthenticated technical probes, used by Docker and Kubernetes: `/health` (liveness) and `/ready` (readiness).")
    ),
    nest(
        (path = "/api/v1", api = V1Doc),
        (path = "/", api = HealthDoc),
        (path = "/", api = ReadyDoc),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

/// Without this modifier, the operations declaring `security(("jwt" = []))` reference a scheme
/// missing from the contract: Swagger UI shows no "Authorize" button and generated clients
/// point to nothing.
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt",
            SecurityScheme::Http(
                Http::builder()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("JWT issued by Core API (`POST /api/v1/auth/login`)."))
                    .build(),
            ),
        )
    }
}

/// Environment variable serving Swagger UI and `/api-docs/openapi.json` when set to `true` or `1`
/// (MAIR-424). Off by default, so a production image does not publish its contract: the dev, ZAP,
/// k6 and integration stacks enable it, the BFFs read the published npm package instead.
pub const SWAGGER_ENABLED_ENV: &str = "SWAGGER_ENABLED";

/// Whether `value` (of [`SWAGGER_ENABLED_ENV`]) enables the documentation routes.
pub fn is_swagger_enabled(value: Option<&str>) -> bool {
    value.is_some_and(|value| {
        let value = value.trim();
        value == "1" || value.eq_ignore_ascii_case("true")
    })
}

/// Reads [`SWAGGER_ENABLED_ENV`] from the environment.
pub fn swagger_enabled_from_env() -> bool {
    is_swagger_enabled(get_env_var(SWAGGER_ENABLED_ENV).as_deref())
}

#[cfg(test)]
mod tests {
    use super::is_swagger_enabled;

    #[test]
    fn swagger_is_off_unless_explicitly_enabled() {
        for value in [
            None,
            Some(""),
            Some("false"),
            Some("0"),
            Some("yes"),
            Some("on"),
        ] {
            assert!(!is_swagger_enabled(value), "{value:?}");
        }
        for value in [Some("true"), Some("TRUE"), Some(" 1 ")] {
            assert!(is_swagger_enabled(value), "{value:?}");
        }
    }
}
