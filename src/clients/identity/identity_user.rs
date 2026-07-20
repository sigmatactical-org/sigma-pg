//! [`IdentityUser`].

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct IdentityUser {
    pub id: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
