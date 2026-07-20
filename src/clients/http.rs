use std::sync::OnceLock;

use reqwest::{Client, RequestBuilder, Response};

use super::internal;

static HTTP: OnceLock<Client> = OnceLock::new();

/// Process-wide HTTP client (connection pooling, TLS session reuse).
#[must_use]
pub fn client() -> &'static Client {
    HTTP.get_or_init(|| {
        Client::builder()
            .user_agent("sigma-clients/0.1")
            .build()
            .expect("reqwest client")
    })
}

/// Attach service-to-service auth (`x-sigma-internal-token` / Bearer) when configured.
#[must_use = "builder does nothing until sent"]
pub fn with_internal_auth(builder: RequestBuilder) -> RequestBuilder {
    match internal::internal_token() {
        Some(token) => builder.header("x-sigma-internal-token", token),
        None => builder,
    }
}

/// Pass 2xx responses through; otherwise read the body and return
/// `Err("{status}: {body}")` for the caller to wrap in its error type.
pub async fn ensure_success(response: Response) -> Result<Response, String> {
    if response.status().is_success() {
        return Ok(response);
    }
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    Err(format!("{status}: {body}"))
}

/// Trim a base URL and guarantee a single trailing slash.
#[must_use]
pub fn normalize_base_url(url: &str) -> String {
    let mut url = url.trim().to_string();
    if !url.ends_with('/') {
        url.push('/');
    }
    url
}

/// Read a base URL from `var` (trimmed, trailing slash normalized), falling
/// back to `default` when the variable is unset or blank.
#[must_use]
pub fn env_url(var: &str, default: &str) -> String {
    std::env::var(var)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| normalize_base_url(&s))
        .unwrap_or_else(|| normalize_base_url(default))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_base_url_guarantees_single_trailing_slash() {
        assert_eq!(
            normalize_base_url("http://127.0.0.1:8081"),
            "http://127.0.0.1:8081/"
        );
        assert_eq!(
            normalize_base_url(" http://127.0.0.1:8081/ "),
            "http://127.0.0.1:8081/"
        );
    }

    #[test]
    fn env_url_prefers_env_and_falls_back_to_default() {
        temp_env::with_var("SIGMA_PG_TEST_BASE_URL", Some(" http://svc:9000 "), || {
            assert_eq!(
                env_url("SIGMA_PG_TEST_BASE_URL", "http://127.0.0.1:8081"),
                "http://svc:9000/"
            );
        });
        temp_env::with_var("SIGMA_PG_TEST_BASE_URL", Some("  "), || {
            assert_eq!(
                env_url("SIGMA_PG_TEST_BASE_URL", "http://127.0.0.1:8081"),
                "http://127.0.0.1:8081/"
            );
        });
        temp_env::with_var("SIGMA_PG_TEST_BASE_URL", None::<&str>, || {
            assert_eq!(
                env_url("SIGMA_PG_TEST_BASE_URL", "http://127.0.0.1:8081"),
                "http://127.0.0.1:8081/"
            );
        });
    }
}
