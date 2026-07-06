/// Shared secret for service-to-service JSON API calls (`SIGMA_INTERNAL_TOKEN`).
#[must_use]
pub fn internal_token() -> Option<String> {
    std::env::var("SIGMA_INTERNAL_TOKEN")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Returns true when the request may access internal JSON/admin APIs.
#[must_use]
pub fn authorize_internal(authorization: Option<&str>, internal_header: Option<&str>) -> bool {
    let Some(expected) = internal_token() else {
        return true;
    };
    if let Some(value) = internal_header
        && value.trim() == expected
    {
        return true;
    }
    authorization
        .and_then(|value| value.strip_prefix("Bearer "))
        .is_some_and(|token| token.trim() == expected)
}
