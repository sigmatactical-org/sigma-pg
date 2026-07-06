use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

use super::http;

#[derive(Debug, Error)]
pub enum StorefrontError {
    #[error("store integration is not configured")]
    NotConfigured,
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("store request failed: {0}")]
    Request(String),
}

#[derive(Debug, Clone, Deserialize)]
struct Listing {
    sku_id: String,
    #[serde(default)]
    price_cents: Option<u64>,
    #[serde(default)]
    visible: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct StorefrontItem {
    listing: Listing,
}

/// Map of catalog SKU id -> unit price in cents, for visible, priced listings.
#[derive(Debug, Clone, Default)]
pub struct PriceBook {
    prices: HashMap<String, u64>,
}

impl PriceBook {
    #[must_use]
    pub fn unit_price_cents(&self, sku_id: &str) -> Option<u64> {
        self.prices.get(sku_id).copied()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.prices.is_empty()
    }
}

pub async fn fetch_prices(store_base_url: Option<&str>) -> Result<PriceBook, StorefrontError> {
    let Some(base) = store_base_url.filter(|s| !s.trim().is_empty()) else {
        return Err(StorefrontError::NotConfigured);
    };
    let mut url = base.trim().to_string();
    if !url.ends_with('/') {
        url.push('/');
    }
    url.push_str("items");
    let response = http::client().get(url).send().await?;
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(StorefrontError::Request(format!("{status}: {body}")));
    }
    let items: Vec<StorefrontItem> = response.json().await?;
    let prices = items
        .into_iter()
        .filter(|item| item.listing.visible)
        .filter_map(|item| {
            item.listing
                .price_cents
                .filter(|cents| *cents > 0)
                .map(|cents| (item.listing.sku_id, cents))
        })
        .collect();
    Ok(PriceBook { prices })
}
