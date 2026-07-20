//! Minimal client for the cart service: consumers read the live item count
//! for their navbar badge, server-side, using the shared guest-cart cookie
//! (`sigma_cart`, shared across subdomains via the cart service's cookie
//! `Domain`).

mod cart_detail;
mod cart_error;
mod cart_line;
mod cart_line_detail;
pub use cart_detail::CartDetail;
pub use cart_error::CartError;
pub(crate) use cart_line::CartLine;
pub(crate) use cart_line_detail::CartLineDetail;

use super::http;

const CART_COOKIE: &str = "sigma_cart";

/// Fetch a cart by id. Returns `Ok(None)` when the cart no longer exists.
pub async fn get_cart(
    base_url: Option<&str>,
    cart_id: &str,
) -> Result<Option<CartDetail>, CartError> {
    let base = base_url
        .filter(|s| !s.trim().is_empty())
        .ok_or(CartError::NotConfigured)?;
    let url = format!("{}carts/{cart_id}", http::normalize_base_url(base));
    let response = http::with_internal_auth(http::client().get(url))
        .send()
        .await?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    let response = http::ensure_success(response)
        .await
        .map_err(CartError::Request)?;
    Ok(Some(response.json().await?))
}

/// Extract the guest cart id from the request `Cookie` header, if present.
#[must_use]
pub fn cart_id_from_cookie(cookie_header: Option<&str>) -> Option<String> {
    cookie_header?.split(';').find_map(|pair| {
        let (name, value) = pair.split_once('=')?;
        (name.trim() == CART_COOKIE)
            .then(|| value.trim().to_string())
            .filter(|v| !v.is_empty())
    })
}

/// Total item count for the nav cart badge (0 when the cart integration is
/// unconfigured or there is no live cart).
pub async fn nav_cart_count(base_url: Option<&str>, cookie_header: Option<&str>) -> u32 {
    if base_url.filter(|s| !s.trim().is_empty()).is_none() {
        return 0;
    }
    let Some(cart_id) = cart_id_from_cookie(cookie_header) else {
        return 0;
    };
    match get_cart(base_url, &cart_id).await {
        Ok(Some(detail)) => detail.item_count(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cart_id_from_cookie_finds_sigma_cart() {
        assert_eq!(
            cart_id_from_cookie(Some("a=b; sigma_cart=cart-1; c=d")),
            Some("cart-1".to_string())
        );
        assert_eq!(cart_id_from_cookie(Some("a=b")), None);
        assert_eq!(cart_id_from_cookie(None), None);
    }
}
