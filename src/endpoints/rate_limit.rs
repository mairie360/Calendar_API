//! Per-user rate limiting of the authenticated `/api` scope (MAIR-425).
//!
//! The APIs are only reached through the BFFs, so every request comes from a handful of pod IPs:
//! a limit per peer IP would throttle all the users of a BFF together. The limiter therefore runs
//! after `JwtMiddleware` and is keyed by the authenticated user id.

use actix_governor::governor::clock::{Clock, DefaultClock, QuantaInstant};
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::governor::NotUntil;
use actix_governor::{
    GovernorConfig, GovernorConfigBuilder, KeyExtractor, SimpleKeyExtractionError,
};
use actix_web::dev::ServiceRequest;
use actix_web::http::header::ContentType;
use actix_web::{HttpMessage, HttpResponse, HttpResponseBuilder};
use mairie360_api_lib::env_manager::get_env_var;
use mairie360_api_lib::security::AuthenticatedUser;

/// Requests per second a user is allowed on average; `0` disables the limiter.
pub const RATE_LIMIT_PER_SECOND_ENV: &str = "RATE_LIMIT_PER_SECOND";
/// Requests a user may send at once before being limited to the average rate.
pub const RATE_LIMIT_BURST_ENV: &str = "RATE_LIMIT_BURST";

pub const DEFAULT_RATE_LIMIT_PER_SECOND: u64 = 10;
pub const DEFAULT_RATE_LIMIT_BURST: u32 = 50;

/// Quota of one user: `burst` requests at once, then `per_second` per second.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimit {
    pub per_second: u64,
    pub burst: u32,
}

impl RateLimit {
    /// Reads the quota from the raw values of [`RATE_LIMIT_PER_SECOND_ENV`] and
    /// [`RATE_LIMIT_BURST_ENV`]: `None` when the rate is `0` (limiter disabled). A missing or
    /// unreadable value falls back to its default.
    pub fn from_values(per_second: Option<&str>, burst: Option<&str>) -> Option<Self> {
        let per_second = per_second
            .and_then(|value| value.trim().parse::<u64>().ok())
            .unwrap_or(DEFAULT_RATE_LIMIT_PER_SECOND);
        let burst = burst
            .and_then(|value| value.trim().parse::<u32>().ok())
            .filter(|burst| *burst > 0)
            .unwrap_or(DEFAULT_RATE_LIMIT_BURST);
        (per_second > 0).then_some(Self { per_second, burst })
    }

    /// Reads the quota from the environment.
    pub fn from_env() -> Option<Self> {
        Self::from_values(
            get_env_var(RATE_LIMIT_PER_SECOND_ENV).as_deref(),
            get_env_var(RATE_LIMIT_BURST_ENV).as_deref(),
        )
    }

    pub fn governor_config(&self) -> GovernorConfig<UserKeyExtractor, NoOpMiddleware> {
        GovernorConfigBuilder::default()
            .key_extractor(UserKeyExtractor)
            .requests_per_second(self.per_second)
            .burst_size(self.burst)
            .finish()
            .expect("a positive rate and burst always build a valid quota")
    }
}

/// Keys the quota by the user id `JwtMiddleware` stored in the request.
#[derive(Debug, Clone, Copy)]
pub struct UserKeyExtractor;

impl KeyExtractor for UserKeyExtractor {
    type Key = u64;
    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        req.extensions()
            .get::<AuthenticatedUser>()
            .map(|user| user.id)
            .ok_or_else(|| {
                log::error!("rate limiter mounted before JwtMiddleware: no authenticated user");
                SimpleKeyExtractionError::new("Internal error.")
            })
    }

    /// `429` with a `Retry-After` header (seconds) and a text body, like the other errors.
    fn exceed_rate_limit_response(
        &self,
        negative: &NotUntil<QuantaInstant>,
        mut response: HttpResponseBuilder,
    ) -> HttpResponse {
        let wait = negative
            .wait_time_from(DefaultClock::default().now())
            .as_secs()
            .max(1);
        response
            .content_type(ContentType::plaintext())
            .insert_header(("Retry-After", wait.to_string()))
            .body(format!("Too many requests, retry in {wait}s."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_quota_defaults_and_can_be_disabled() {
        assert_eq!(
            RateLimit::from_values(None, None),
            Some(RateLimit {
                per_second: DEFAULT_RATE_LIMIT_PER_SECOND,
                burst: DEFAULT_RATE_LIMIT_BURST
            })
        );
        assert_eq!(
            RateLimit::from_values(Some(" 3 "), Some("7")),
            Some(RateLimit {
                per_second: 3,
                burst: 7
            })
        );
        assert_eq!(
            RateLimit::from_values(Some("many"), Some("0")),
            Some(RateLimit {
                per_second: DEFAULT_RATE_LIMIT_PER_SECOND,
                burst: DEFAULT_RATE_LIMIT_BURST
            })
        );
        assert_eq!(RateLimit::from_values(Some("0"), None), None);
    }
}
