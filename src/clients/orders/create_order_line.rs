//! [`CreateOrderLine`].

use serde::{Deserialize, Serialize};

/// A priced line submitted when creating an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderLine {
    pub sku_id: String,
    pub sku_code: String,
    pub name: String,
    pub quantity: u32,
    pub unit_price_cents: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_total_cents: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deposit_cents: Option<u64>,
}
