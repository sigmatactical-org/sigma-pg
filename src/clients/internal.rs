/// Shared secret for service-to-service JSON API calls (`SIGMA_INTERNAL_TOKEN`).
#[must_use]
pub fn internal_token() -> Option<String> {
    std::env::var("SIGMA_INTERNAL_TOKEN")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn internal_auth_disabled() -> bool {
    matches!(
        std::env::var("SIGMA_INTERNAL_AUTH_DISABLED")
            .ok()
            .as_deref(),
        Some("1" | "true" | "TRUE" | "yes" | "YES")
    )
}

/// Returns true when the request may access internal JSON/admin APIs.
#[must_use]
pub fn authorize_internal(authorization: Option<&str>, internal_header: Option<&str>) -> bool {
    if internal_auth_disabled() {
        return true;
    }
    let Some(expected) = internal_token() else {
        return false;
    };
    if let Some(value) = internal_header
        && value.trim() == expected
    {
        return true;
    }
    authorization
        .and_then(|value| value.strip_prefix("Bearer "))
        .is_some_and(|token| token.trim() == expected)
}

/// Dev/test fixture token (32+ chars). Also used in platform dev ConfigMaps.
pub const DEV_INTERNAL_TOKEN: &str = "dev-internal-token-32chars-minimum!!";

/// Test fixture token (same value as [`DEV_INTERNAL_TOKEN`] for local/CI tests).
pub const TEST_INTERNAL_TOKEN: &str = DEV_INTERNAL_TOKEN;

/// Set [`TEST_INTERNAL_TOKEN`] when unset (CI uses `--test-threads=1`).
pub fn ensure_test_internal_token() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        if internal_token().is_none() {
            // SAFETY: tests run with `--test-threads=1` in CI.
            unsafe {
                std::env::set_var("SIGMA_INTERNAL_TOKEN", TEST_INTERNAL_TOKEN);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn denies_when_token_unset_and_auth_not_disabled() {
        temp_env::with_vars(
            [
                ("SIGMA_INTERNAL_TOKEN", None::<&str>),
                ("SIGMA_INTERNAL_AUTH_DISABLED", None::<&str>),
            ],
            || {
                assert!(!authorize_internal(None, None));
                assert!(!authorize_internal(Some("Bearer wrong"), Some("wrong")));
            },
        );
    }

    #[test]
    fn allows_matching_bearer_token() {
        temp_env::with_vars(
            [
                (
                    "SIGMA_INTERNAL_TOKEN",
                    Some("secret-token-value-32chars-min!!"),
                ),
                ("SIGMA_INTERNAL_AUTH_DISABLED", None::<&str>),
            ],
            || {
                assert!(authorize_internal(
                    Some("Bearer secret-token-value-32chars-min!!"),
                    None
                ));
                assert!(authorize_internal(
                    None,
                    Some("secret-token-value-32chars-min!!")
                ));
            },
        );
    }

    #[test]
    fn allows_all_when_explicitly_disabled() {
        temp_env::with_vars(
            [
                ("SIGMA_INTERNAL_TOKEN", None::<&str>),
                ("SIGMA_INTERNAL_AUTH_DISABLED", Some("true")),
            ],
            || {
                assert!(authorize_internal(None, None));
            },
        );
    }
}
