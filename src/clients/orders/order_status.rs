//! [`OrderStatus`].

use serde::{Deserialize, Serialize};

/// Lifecycle for a committed customer order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    PendingDeposit,
    DepositPaid,
    InBuild,
    Shipped,
    Cancelled,
}
impl OrderStatus {
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim() {
            "pending_deposit" => Some(Self::PendingDeposit),
            "deposit_paid" => Some(Self::DepositPaid),
            "in_build" => Some(Self::InBuild),
            "shipped" => Some(Self::Shipped),
            "cancelled" => Some(Self::Cancelled),
            _ => None,
        }
    }
}
