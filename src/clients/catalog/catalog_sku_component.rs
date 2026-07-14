//! [`CatalogSkuComponent`].

#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogSkuComponent {
    pub sku_id: String,
    pub quantity: u32,
}
