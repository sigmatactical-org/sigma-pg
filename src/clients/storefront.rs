mod listing;
mod price_book;
mod storefront_error;
mod storefront_item;
pub(crate) use listing::Listing;
pub use price_book::PriceBook;
pub use storefront_error::StorefrontError;
pub(crate) use storefront_item::StorefrontItem;

use super::http;

pub async fn fetch_prices(store_base_url: Option<&str>) -> Result<PriceBook, StorefrontError> {
    let Some(base) = store_base_url.filter(|s| !s.trim().is_empty()) else {
        return Err(StorefrontError::NotConfigured);
    };
    let url = format!("{}items", http::normalize_base_url(base));
    let response = http::with_internal_auth(http::client().get(url))
        .send()
        .await?;
    let response = http::ensure_success(response)
        .await
        .map_err(StorefrontError::Request)?;
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
