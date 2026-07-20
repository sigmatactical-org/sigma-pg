//! Shared warp JSON-API plumbing: error responses, store-error mapping, and
//! the internal service-to-service auth filter.

mod error_body;
mod store_error;
pub use error_body::ErrorBody;
pub use store_error::StoreError;

use warp::http::StatusCode;
use warp::reply::Response;
use warp::{Filter, Rejection, Reply};

use crate::clients::internal;

/// JSON error response `{ "error": message }` with the given status.
#[must_use]
pub fn json_error(status: StatusCode, message: impl Into<String>) -> Response {
    warp::reply::with_status(
        warp::reply::json(&ErrorBody {
            error: message.into(),
        }),
        status,
    )
    .into_response()
}

/// Map a [`StoreError`] to the HTTP status of its JSON error response.
#[must_use]
pub fn store_error_status(err: &StoreError) -> StatusCode {
    match err {
        StoreError::NotFound(_) => StatusCode::NOT_FOUND,
        StoreError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        StoreError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// Require internal service auth (`Authorization: Bearer` or
/// `x-sigma-internal-token`), rejecting unauthorized requests as not-found.
pub fn internal_auth() -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::header::optional::<String>("authorization")
        .and(warp::header::optional::<String>("x-sigma-internal-token"))
        .and_then(
            |authorization: Option<String>, internal_token: Option<String>| async move {
                if internal::authorize_internal(authorization.as_deref(), internal_token.as_deref())
                {
                    Ok::<_, Rejection>(())
                } else {
                    Err(warp::reject::not_found())
                }
            },
        )
        .untuple_one()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_error_maps_to_expected_statuses() {
        assert_eq!(
            store_error_status(&StoreError::NotFound("address")),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            store_error_status(&StoreError::InvalidInput("bad".to_string())),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            store_error_status(&StoreError::Database(anyhow::anyhow!("boom"))),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn not_found_message_names_the_entity() {
        assert_eq!(
            StoreError::NotFound("payment method").to_string(),
            "payment method not found"
        );
    }
}
