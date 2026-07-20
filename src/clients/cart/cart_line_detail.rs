//! [`CartLineDetail`].

use serde::Deserialize;

use super::CartLine;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CartLineDetail {
    pub(crate) line: CartLine,
}
