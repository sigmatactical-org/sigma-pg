//! [`CatalogSku`].

use serde::{Deserialize, Serialize};

use super::{CatalogSkuComponent, CatalogSkuKind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogSku {
    pub id: String,
    pub sku_code: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub kind: CatalogSkuKind,
    pub active: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CatalogSkuComponent>,
    pub updated_at: String,
}
