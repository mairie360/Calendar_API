//! JSON body extractor whose `400` names the faulty field (MAIR-481).

use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use futures_util::future::LocalBoxFuture;
use serde::de::DeserializeOwned;

use crate::endpoints::error::ApiError;

/// Like `web::Json<T>`, but a body that does not match `T` is a `400` whose text gives the path
/// of the field at fault (`Invalid `recurrence.interval`: invalid value: integer `-1`, expected
/// u32.`) instead of a line and column. A body that is not JSON at all keeps actix's `400`.
#[derive(Debug)]
pub struct JsonBody<T>(pub T);

impl<T> JsonBody<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for JsonBody<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// Turns a deserialization error into a `400` naming the field (`body` for the whole object,
/// e.g. a missing field).
pub fn body_error(error: serde_path_to_error::Error<serde_json::Error>) -> ApiError {
    let path = error.path().to_string();
    let field = if path == "." { "body" } else { path.as_str() };
    ApiError::invalid_field(field, &error.into_inner().to_string())
}

impl<T: DeserializeOwned + 'static> FromRequest for JsonBody<T> {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let value = web::Json::<serde_json::Value>::from_request(req, payload);
        Box::pin(async move {
            let value = value.await?.into_inner();
            serde_path_to_error::deserialize(value)
                .map(JsonBody)
                .map_err(|error| body_error(error).into())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    #[allow(dead_code)]
    struct Body {
        name: String,
        nested: Option<Nested>,
    }

    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code)]
    struct Nested {
        interval: u32,
    }

    fn error_of(json: serde_json::Value) -> String {
        body_error(serde_path_to_error::deserialize::<_, Body>(json).unwrap_err()).to_string()
    }

    #[test]
    fn errors_name_the_field() {
        assert!(error_of(serde_json::json!({ "name": 12 })).starts_with("Invalid `name`: "));
        assert!(
            error_of(serde_json::json!({ "name": "a", "nested": { "interval": -1 } }))
                .starts_with("Invalid `nested.interval`: ")
        );
        assert_eq!(
            error_of(serde_json::json!({})),
            "Invalid `body`: missing field `name`."
        );
        assert!(error_of(serde_json::json!({ "name": "a", "colour": 1 }))
            .contains("unknown field `colour`"));
    }
}
