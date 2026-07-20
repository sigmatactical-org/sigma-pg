//! [`IssuerParts`].

/// URLs derived from the OIDC issuer (token + admin API bases).
pub(crate) struct IssuerParts {
    pub(crate) admin_base: String,
    pub(crate) realm: String,
}
