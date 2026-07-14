//! [`StorefrontItem`].

#[allow(unused_imports)]
use super::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct StorefrontItem {
    pub(crate) listing: Listing,
}
