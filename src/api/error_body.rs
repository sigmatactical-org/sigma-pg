//! [`ErrorBody`].

/// JSON error payload `{ "error": "..." }` returned by internal APIs.
#[derive(Debug, serde::Serialize)]
pub struct ErrorBody {
    pub error: String,
}
