//! [`PriceBook`].

#[allow(unused_imports)]
use super::*;
use std::collections::HashMap;

/// Map of catalog SKU id -> unit price in cents, for visible, priced listings.
#[derive(Debug, Clone, Default)]
pub struct PriceBook {
    pub(crate) prices: HashMap<String, u64>,
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
