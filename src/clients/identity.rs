mod identity_error;
mod identity_user;
mod issuer_parts;
mod keycloak_user;
mod token_response;
pub use identity_error::IdentityError;
pub use identity_user::IdentityUser;
pub(crate) use issuer_parts::IssuerParts;
pub(crate) use keycloak_user::KeycloakUser;
pub(crate) use token_response::TokenResponse;

use super::http;

fn parse_issuer(issuer: &str) -> Result<IssuerParts, IdentityError> {
    let issuer = issuer.trim().trim_end_matches('/');
    let Some((base, realm)) = issuer.rsplit_once("/realms/") else {
        return Err(IdentityError::InvalidIssuer(
            "expected URL ending with /realms/{realm}".to_string(),
        ));
    };
    if realm.is_empty() {
        return Err(IdentityError::InvalidIssuer(
            "missing realm name".to_string(),
        ));
    }
    Ok(IssuerParts {
        admin_base: base.to_string(),
        realm: realm.to_string(),
    })
}

async fn fetch_access_token(
    issuer: &IssuerParts,
    client_id: &str,
    client_secret: &str,
) -> Result<String, IdentityError> {
    let token_url = format!(
        "{}/realms/{}/protocol/openid-connect/token",
        issuer.admin_base, issuer.realm
    );
    let response = http::client()
        .post(token_url)
        .form(&[
            ("grant_type", "client_credentials"),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(IdentityError::Token(format!("{status}: {body}")));
    }

    let token: TokenResponse = response.json().await?;
    Ok(token.access_token)
}

fn display_name_for_user(user: &KeycloakUser) -> String {
    let first = user.first_name.as_deref().unwrap_or("").trim();
    let last = user.last_name.as_deref().unwrap_or("").trim();
    let full = format!("{first} {last}").trim().to_string();
    if !full.is_empty() {
        return full;
    }
    user.email
        .clone()
        .or_else(|| user.username.clone())
        .unwrap_or_else(|| user.id.clone())
}

/// Pull enabled realm users from Keycloak Admin API.
pub async fn fetch_users(
    issuer_url: Option<&str>,
    client_id: Option<&str>,
    client_secret: Option<&str>,
) -> Result<Vec<IdentityUser>, IdentityError> {
    let issuer_url = issuer_url
        .filter(|s| !s.trim().is_empty())
        .ok_or(IdentityError::NotConfigured)?;
    let client_id = client_id
        .filter(|s| !s.trim().is_empty())
        .ok_or(IdentityError::NotConfigured)?;
    let client_secret = client_secret
        .filter(|s| !s.trim().is_empty())
        .ok_or(IdentityError::NotConfigured)?;
    let issuer = parse_issuer(issuer_url)?;

    let token = fetch_access_token(&issuer, client_id, client_secret).await?;

    let users_url = format!(
        "{}/admin/realms/{}/users?max=1000",
        issuer.admin_base, issuer.realm
    );
    let response = http::client()
        .get(users_url)
        .bearer_auth(token)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(IdentityError::Users(format!("{status}: {body}")));
    }

    let users: Vec<KeycloakUser> = response.json().await?;
    Ok(users
        .into_iter()
        .filter(|u| u.enabled.unwrap_or(true))
        .filter(|u| {
            !u.username
                .as_deref()
                .is_some_and(|n| n.starts_with("service-account-"))
        })
        .map(|u| IdentityUser {
            id: u.id.clone(),
            display_name: display_name_for_user(&u),
            email: u.email.filter(|e| !e.is_empty()),
        })
        .collect())
}

#[must_use]
pub fn user_by_id<'a>(users: &'a [IdentityUser], id: &str) -> Option<&'a IdentityUser> {
    users.iter().find(|u| u.id == id)
}
