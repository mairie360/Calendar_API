use actix_web::{middleware, web, App, HttpServer};

use calendar_api::database::pg_url::build_pg_url;
use calendar_api::database::service::readiness::{check_dependencies, Dependency};
use calendar_api::endpoints::swagger::{swagger_enabled_from_env, ApiDoc};
use calendar_api::endpoints::{config, health, ready};

use mairie360_api_lib::env_manager::get_critical_env_var;
use mairie360_api_lib::security::JwtMiddleware;
use mairie360_api_lib::state::AppState;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Attempts made to reach PostgreSQL at startup, `STARTUP_DB_RETRY_DELAY` apart.
const STARTUP_DB_ATTEMPTS: u32 = 15;
const STARTUP_DB_RETRY_DELAY: std::time::Duration = std::time::Duration::from_secs(2);

/// Refuses to start without PostgreSQL (MAIR-423): `AppState::new` only logs a failed connection,
/// and the API would then answer every request with a `500` while looking healthy. Redis is not
/// required to start: `/ready` reports it.
async fn wait_for_database(state: &AppState) -> std::io::Result<()> {
    for attempt in 1..=STARTUP_DB_ATTEMPTS {
        match check_dependencies(state).await {
            Ok(()) | Err(Dependency::Redis) => return Ok(()),
            Err(Dependency::Postgres) => log::warn!(
                "PostgreSQL unreachable (attempt {attempt}/{STARTUP_DB_ATTEMPTS}), retrying"
            ),
        }
        tokio::time::sleep(STARTUP_DB_RETRY_DELAY).await;
    }
    log::error!("PostgreSQL still unreachable after {STARTUP_DB_ATTEMPTS} attempts, exiting");
    Err(std::io::Error::other("PostgreSQL unreachable at startup"))
}

//                                        -- MAIN FUNCTION --

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Structured logs (level, timestamp, target) for the handlers and actix's access log; the level
    // is set by `RUST_LOG` (`info` by default, `calendar_api=debug` to trace the database layer).
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let redis_url = get_critical_env_var("REDIS_URL");
    let db_user = get_critical_env_var("DB_USER");
    let db_password = get_critical_env_var("DB_PASSWORD");
    let db_host = get_critical_env_var("DB_HOST");
    let db_port = get_critical_env_var("DB_PORT");
    let db_name = get_critical_env_var("DB_NAME");
    let pg_url = build_pg_url(&db_user, &db_password, &db_host, &db_port, &db_name);
    let state = AppState::new(redis_url, pg_url).await;
    wait_for_database(&state).await?;
    let data = web::Data::new(state);
    let host = get_critical_env_var("HOST");
    let port = get_critical_env_var("PORT");
    let bind_address = format!("{}:{}", host, port);

    let swagger_enabled = swagger_enabled_from_env();
    log::info!("Swagger UI and /api-docs/openapi.json served: {swagger_enabled}");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            // Every response is JSON or plain text: forbid browsers from sniffing it as HTML.
            .wrap(middleware::DefaultHeaders::new().add(("X-Content-Type-Options", "nosniff")))
            // 1. Swagger UI and the OpenAPI document, only when SWAGGER_ENABLED is set (MAIR-424).
            .configure(|cfg| {
                if swagger_enabled {
                    cfg.service(
                        SwaggerUi::new("/swagger-ui/{_:.*}")
                            .url("/api-docs/openapi.json", ApiDoc::openapi()),
                    );
                }
            })
            // 2. Public probes.
            .service(health::health)
            .service(ready::ready)
            // 3. Routes protected by a JWT.
            .service(web::scope("/api").wrap(JwtMiddleware).configure(config))
    })
    .bind(bind_address)?;

    let addr = server.addrs().first().copied();
    tokio::spawn(async move {
        if let Some(addr) = addr {
            log::info!("Server listening on http://{addr}");
        }
    });

    server.run().await
}
