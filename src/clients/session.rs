mod identity_status;
mod session_error;
pub use identity_status::IdentityStatus;
pub use session_error::SessionError;

use super::http;

/// Resolve the signed-in user from the identity BFF using browser session cookies.
pub async fn fetch_identity_status(
    identity_public_base_url: &str,
    cookie_header: Option<&str>,
) -> Result<Option<IdentityStatus>, SessionError> {
    let Some(cookie_header) = cookie_header.filter(|value| !value.trim().is_empty()) else {
        return Ok(None);
    };
    if identity_public_base_url.trim().is_empty() {
        return Err(SessionError::NotConfigured);
    }
    let url = status_url(identity_public_base_url);

    let response = http::client()
        .get(url)
        .header("cookie", cookie_header)
        .send()
        .await?;
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(SessionError::Request(format!("{status}: {body}")));
    }

    let status = response.json::<IdentityStatus>().await?;
    Ok(status.authenticated.then_some(status))
}

fn status_url(identity_base_url: &str) -> String {
    format!("{}/auth/status", identity_base_url.trim_end_matches('/'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_url_inserts_separator_regardless_of_trailing_slash() {
        assert_eq!(
            status_url("http://identity.sigma-dev.svc.cluster.local/"),
            "http://identity.sigma-dev.svc.cluster.local/auth/status"
        );
        assert_eq!(
            status_url("http://identity.sigma-dev.svc.cluster.local"),
            "http://identity.sigma-dev.svc.cluster.local/auth/status"
        );
    }
}
