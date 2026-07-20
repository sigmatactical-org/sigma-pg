//! [`Order`].

use serde::{Deserialize, Serialize};

use super::{OrderLine, OrderStatus};

/// A committed customer sales order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub cart_id: String,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub lines: Vec<OrderLine>,
    pub subtotal_cents: u64,
    pub deposit_cents: u64,
    pub status: OrderStatus,
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
    pub created_at: String,
    pub updated_at: String,
}
