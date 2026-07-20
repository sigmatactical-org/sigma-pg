//! [`StorefrontItem`].

use serde::Deserialize;

use super::Listing;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct StorefrontItem {
    pub(crate) listing: Listing,
}
