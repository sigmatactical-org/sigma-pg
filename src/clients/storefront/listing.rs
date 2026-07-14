//! [`Listing`].

#[allow(unused_imports)]
use super::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Listing {
    pub(crate) sku_id: String,
    #[serde(default)]
    pub(crate) price_cents: Option<u64>,
    #[serde(default)]
    pub(crate) visible: bool,
}
