use actix_governor::Governor;
use actix_web::{middleware, web, App, HttpServer};

use calendar_api::database::pg_url::build_pg_url;
use calendar_api::endpoints::rate_limit::{
    RateLimit, DEFAULT_RATE_LIMIT_BURST, DEFAULT_RATE_LIMIT_PER_SECOND,
};
use calendar_api::endpoints::swagger::{swagger_enabled_from_env, ApiDoc};
use calendar_api::endpoints::{config, health, ready};
use calendar_api::telemetry;

use mairie360_api_lib::env_manager::get_critical_env_var;
use mairie360_api_lib::security::JwtMiddleware;
use mairie360_api_lib::state::AppState;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

//                                        -- MAIN FUNCTION --

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Structured logs (level, timestamp, target) for the handlers and actix's access log; the level
    // is set by `RUST_LOG` (`info` by default, `calendar_api=debug` to trace the database layer).
    // Plus the trace export when `OTEL_EXPORTER_OTLP_ENDPOINT` is set (MAIR-503); flushed on drop.
    let _telemetry = telemetry::init();

    let redis_url = get_critical_env_var("REDIS_URL");
    let db_user = get_critical_env_var("DB_USER");
    let db_password = get_critical_env_var("DB_PASSWORD");
    let db_host = get_critical_env_var("DB_HOST");
    let db_port = get_critical_env_var("DB_PORT");
    let db_name = get_critical_env_var("DB_NAME");
    let pg_url = build_pg_url(&db_user, &db_password, &db_host, &db_port, &db_name);
    // Panics when PostgreSQL stays unreachable for `DB_CONNECT_TIMEOUT` seconds (MAIR-423): the pod
    // crashes and is restarted instead of answering `500` on every route.
    let state = AppState::new(redis_url, pg_url).await;
    let data = web::Data::new(state);
    let host = get_critical_env_var("HOST");
    let port = get_critical_env_var("PORT");
    let bind_address = format!("{}:{}", host, port);

    let swagger_enabled = swagger_enabled_from_env();
    log::info!("Swagger UI and /api-docs/openapi.json served: {swagger_enabled}");

    // Per-user quota of the /api scope (MAIR-425); RATE_LIMIT_PER_SECOND=0 disables it.
    let rate_limit = RateLimit::from_env();
    match rate_limit {
        Some(limit) => log::info!(
            "Rate limit: {} requests/s per user, bursts of {}",
            limit.per_second,
            limit.burst
        ),
        None => log::warn!("Rate limit disabled (RATE_LIMIT_PER_SECOND=0)"),
    }
    let governor = rate_limit
        .unwrap_or(RateLimit {
            per_second: DEFAULT_RATE_LIMIT_PER_SECOND,
            burst: DEFAULT_RATE_LIMIT_BURST,
        })
        .governor_config();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            // One span per request (MAIR-503), including those refused by `JwtMiddleware`; it
            // continues the `traceparent` of the BFF.
            .wrap(tracing_actix_web::TracingLogger::default())
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
            // The last `wrap` runs first: the JWT is checked, then the caller's quota.
            .service(
                web::scope("/api")
                    .wrap(middleware::Condition::new(
                        rate_limit.is_some(),
                        Governor::new(&governor),
                    ))
                    .wrap(JwtMiddleware)
                    .configure(config),
            )
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
