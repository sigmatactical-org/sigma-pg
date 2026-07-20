//! [`CreateOrder`].

use serde::{Deserialize, Serialize};

use super::{CreateOrderLine, OrderStatus};

/// Input for `POST /orders`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrder {
    pub cart_id: String,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub lines: Vec<CreateOrderLine>,
    /// Preserve legacy reservation id during cart migration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtotal_cents: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deposit_cents: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charge_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms_accepted_at: Option<String>,
}
