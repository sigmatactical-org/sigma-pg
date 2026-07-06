use std::sync::OnceLock;

use reqwest::Client;

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
