mod catalog_error;
mod catalog_sku;
mod catalog_sku_component;
mod catalog_sku_kind;
pub use catalog_error::CatalogError;
pub use catalog_sku::CatalogSku;
pub use catalog_sku_component::CatalogSkuComponent;
pub use catalog_sku_kind::CatalogSkuKind;

use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use super::http;

const CACHE_TTL: Duration = Duration::from_secs(30);

type SkuCacheEntry = Option<(Instant, Vec<CatalogSku>)>;

static SKU_CACHE: OnceLock<Mutex<SkuCacheEntry>> = OnceLock::new();

fn cache() -> &'static Mutex<SkuCacheEntry> {
    SKU_CACHE.get_or_init(|| Mutex::new(None))
}

async fn fetch_skus_uncached(base_url: &str) -> Result<Vec<CatalogSku>, CatalogError> {
    let url = format!("{}skus", normalize_base(base_url));
    let response = http::with_internal_auth(http::client().get(url))
        .send()
        .await?;
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(CatalogError::Request(format!("{status}: {body}")));
    }
    response.json().await.map_err(CatalogError::from)
}

/// Pull all SKUs from the catalog service (cached briefly per process).
pub async fn fetch_skus(base_url: Option<&str>) -> Result<Vec<CatalogSku>, CatalogError> {
    let Some(base) = base_url.filter(|s| !s.trim().is_empty()) else {
        return Err(CatalogError::NotConfigured);
    };
    let key = normalize_base(base);
    {
        let guard = cache().lock().expect("catalog cache lock");
        if let Some((fetched_at, skus)) = guard.as_ref()
            && fetched_at.elapsed() < CACHE_TTL
        {
            return Ok(skus.clone());
        }
    }
    let skus = fetch_skus_uncached(&key).await?;
    *cache().lock().expect("catalog cache lock") = Some((Instant::now(), skus.clone()));
    Ok(skus)
}

/// Validate that a catalog SKU id exists and is active.
pub fn validate_sku_id(skus: &[CatalogSku], sku_id: &str) -> Result<(), CatalogError> {
    skus.iter()
        .find(|s| s.id == sku_id && s.active)
        .map(|_| ())
        .ok_or_else(|| CatalogError::Request(format!("catalog sku not found: {sku_id}")))
}

#[must_use]
pub fn sku_by_id<'a>(skus: &'a [CatalogSku], id: &str) -> Option<&'a CatalogSku> {
    skus.iter().find(|s| s.id == id)
}

fn normalize_base(url: &str) -> String {
    let mut url = url.trim().to_string();
    if !url.ends_with('/') {
        url.push('/');
    }
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_rejects_inactive_sku() {
        let skus = vec![CatalogSku {
            id: "a".to_string(),
            sku_code: "X".to_string(),
            name: "X".to_string(),
            description: None,
            category: None,
            kind: CatalogSkuKind::Simple,
            active: false,
            components: vec![],
            updated_at: "now".to_string(),
        }];
        assert!(validate_sku_id(&skus, "a").is_err());
    }
}
