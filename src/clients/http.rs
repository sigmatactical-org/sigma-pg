use std::sync::OnceLock;

use reqwest::{Client, RequestBuilder};

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
