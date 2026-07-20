//! Wire DTOs and HTTP client for the orders service
//! (`POST /orders`, `GET /orders/{id}`).

mod create_order;
mod create_order_line;
mod order;
mod order_error;
mod order_line;
mod order_status;
pub use create_order::CreateOrder;
pub use create_order_line::CreateOrderLine;
pub use order::Order;
pub use order_error::OrderError;
pub use order_line::OrderLine;
pub use order_status::OrderStatus;

use super::http;

fn orders_base(base_url: Option<&str>) -> Result<String, OrderError> {
    base_url
        .filter(|s| !s.trim().is_empty())
        .map(http::normalize_base_url)
        .ok_or(OrderError::NotConfigured)
}

/// Create a committed order in the orders service.
pub async fn create_order(base_url: Option<&str>, input: &CreateOrder) -> Result<Order, OrderError> {
    let url = format!("{}orders", orders_base(base_url)?);
    let response = http::with_internal_auth(http::client().post(url).json(input))
        .send()
        .await
        .map_err(|e| OrderError::Request(e.to_string()))?;
    let response = http::ensure_success(response)
        .await
        .map_err(OrderError::Request)?;
    response
        .json()
        .await
        .map_err(|e| OrderError::Request(e.to_string()))
}

/// Fetch one order by id. Returns `Ok(None)` when the order does not exist.
pub async fn get_order(base_url: Option<&str>, order_id: &str) -> Result<Option<Order>, OrderError> {
    let url = format!("{}orders/{order_id}", orders_base(base_url)?);
    let response = http::with_internal_auth(http::client().get(url))
        .send()
        .await
        .map_err(|e| OrderError::Request(e.to_string()))?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    let response = http::ensure_success(response)
        .await
        .map_err(OrderError::Request)?;
    response
        .json()
        .await
        .map(Some)
        .map_err(|e| OrderError::Request(e.to_string()))
}
