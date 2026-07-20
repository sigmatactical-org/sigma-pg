mod catalog_error;
mod catalog_sku;
mod catalog_sku_component;
mod catalog_sku_kind;
pub use catalog_error::CatalogError;
pub use catalog_sku::CatalogSku;
pub use catalog_sku_component::CatalogSkuComponent;
pub use catalog_sku_kind::CatalogSkuKind;

use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use super::http;

const CACHE_TTL: Duration = Duration::from_secs(30);

/// Cached SKU list keyed by the normalized base URL it was fetched from.
type SkuCacheEntry = Option<(String, Instant, Arc<Vec<CatalogSku>>)>;

static SKU_CACHE: OnceLock<Mutex<SkuCacheEntry>> = OnceLock::new();

fn cache() -> &'static Mutex<SkuCacheEntry> {
    SKU_CACHE.get_or_init(|| Mutex::new(None))
}

async fn fetch_skus_uncached(base_url: &str) -> Result<Vec<CatalogSku>, CatalogError> {
    let url = format!("{base_url}skus");
    let response = http::with_internal_auth(http::client().get(url))
        .send()
        .await?;
    let response = http::ensure_success(response)
        .await
        .map_err(CatalogError::Request)?;
    response.json().await.map_err(CatalogError::from)
}

/// Pull all SKUs from the catalog service (cached briefly per process,
/// per base URL).
pub async fn fetch_skus(base_url: Option<&str>) -> Result<Arc<Vec<CatalogSku>>, CatalogError> {
    let Some(base) = base_url.filter(|s| !s.trim().is_empty()) else {
        return Err(CatalogError::NotConfigured);
    };
    let key = http::normalize_base_url(base);
    {
        let guard = cache().lock().expect("catalog cache lock");
        if let Some((cached_url, fetched_at, skus)) = guard.as_ref()
            && *cached_url == key
            && fetched_at.elapsed() < CACHE_TTL
        {
            return Ok(Arc::clone(skus));
        }
    }
    let skus = Arc::new(fetch_skus_uncached(&key).await?);
    *cache().lock().expect("catalog cache lock") = Some((key, Instant::now(), Arc::clone(&skus)));
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
