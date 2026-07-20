//! Client for the addresses service's internal-token-gated JSON API
//! (`GET /api/users/{user_id}/addresses` and
//! `GET /api/users/{user_id}/addresses/{id}`), used to populate checkout and
//! billing dropdowns and to validate that a submitted address id actually
//! belongs to the caller. Addresses and its consumers are independently owned
//! services communicating over HTTP+JSON only — this module defines its own
//! minimal [`AddressSummary`] rather than depending on the addresses crate.

mod address_summary;
mod addresses_client_error;
pub use address_summary::AddressSummary;
pub use addresses_client_error::AddressesClientError;

use super::http;

fn build_addresses_url(base: &str, path: &str) -> String {
    format!("{base}{}", path.trim_start_matches('/'))
}

fn addresses_url(base_url: Option<&str>, path: &str) -> Result<String, AddressesClientError> {
    let base = base_url.filter(|s| !s.trim().is_empty()).ok_or_else(|| {
        AddressesClientError::Request("addresses service not configured".to_string())
    })?;
    Ok(build_addresses_url(&http::normalize_base_url(base), path))
}

/// List `user_id`'s addresses in `category` (e.g. `billing`, `shipping`).
pub async fn list_addresses(
    base_url: Option<&str>,
    user_id: &str,
    category: &str,
) -> Result<Vec<AddressSummary>, AddressesClientError> {
    let url = addresses_url(
        base_url,
        &format!("api/users/{user_id}/addresses?category={category}"),
    )?;
    let response = http::with_internal_auth(http::client().get(url))
        .send()
        .await?;
    let response = http::ensure_success(response)
        .await
        .map_err(AddressesClientError::Request)?;
    Ok(response.json().await?)
}

/// Fetch one address scoped to `user_id`, returning `Ok(None)` when it
/// doesn't exist or doesn't belong to `user_id`.
pub async fn get_address(
    base_url: Option<&str>,
    user_id: &str,
    id: &str,
) -> Result<Option<AddressSummary>, AddressesClientError> {
    let url = addresses_url(base_url, &format!("api/users/{user_id}/addresses/{id}"))?;
    let response = http::with_internal_auth(http::client().get(url))
        .send()
        .await?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    let response = http::ensure_success(response)
        .await
        .map_err(AddressesClientError::Request)?;
    Ok(Some(response.json().await?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_addresses_url_joins_base_and_path() {
        assert_eq!(
            build_addresses_url(
                "http://addresses.internal:8089/",
                "api/users/user-1/addresses"
            ),
            "http://addresses.internal:8089/api/users/user-1/addresses"
        );
    }

    #[test]
    fn build_addresses_url_strips_leading_slash_from_path() {
        assert_eq!(
            build_addresses_url(
                "http://127.0.0.1:8089/",
                "/api/users/user-1/addresses/addr-1"
            ),
            "http://127.0.0.1:8089/api/users/user-1/addresses/addr-1"
        );
    }

    #[test]
    fn short_summary_combines_line1_and_city() {
        let address = AddressSummary {
            id: "addr-1".to_string(),
            label: None,
            line1: "123 Main St".to_string(),
            city: "Springfield".to_string(),
            region: None,
            postal_code: "62704".to_string(),
            country: "US".to_string(),
            category: "billing".to_string(),
            is_default: false,
        };
        assert_eq!(address.short_summary(), "123 Main St, Springfield");
        assert!(address.is_billing());
    }

    #[test]
    fn is_billing_rejects_shipping_category() {
        let address = AddressSummary {
            id: "addr-1".to_string(),
            label: None,
            line1: "123 Main St".to_string(),
            city: "Springfield".to_string(),
            region: None,
            postal_code: "62704".to_string(),
            country: "US".to_string(),
            category: "shipping".to_string(),
            is_default: false,
        };
        assert!(!address.is_billing());
    }

    #[test]
    fn address_summary_deserializes_from_addresses_api_json() {
        let json = r#"{
            "id": "addr-1",
            "user_id": "user-1",
            "category": "billing",
            "line1": "123 Main St",
            "city": "Springfield",
            "postal_code": "62704",
            "country": "US",
            "is_default": true,
            "updated_at": "2026-01-01T00:00:00Z"
        }"#;
        let address: AddressSummary = serde_json::from_str(json).unwrap();
        assert_eq!(address.id, "addr-1");
        assert_eq!(address.line1, "123 Main St");
        assert!(address.is_billing());
        assert!(address.is_default);
    }
}
