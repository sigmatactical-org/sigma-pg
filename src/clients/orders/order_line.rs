//! [`OrderLine`].

use serde::{Deserialize, Serialize};

/// A single line captured on an order at checkout time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderLine {
    pub sku_id: String,
    pub sku_code: String,
    pub name: String,
    pub quantity: u32,
    pub unit_price_cents: u64,
    pub line_total_cents: u64,
    pub deposit_cents: u64,
}
