//! Form-input helpers shared by Sigma web/admin services.

/// Trim `value` and return `None` when nothing remains.
#[must_use]
pub fn empty_to_none(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Trim `value`, rejecting empty input with `"{field} is required"`.
pub fn required(value: String, field: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(format!("{field} is required"))
    } else {
        Ok(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_to_none_trims_and_drops_blank() {
        assert_eq!(empty_to_none("  hi  ".to_string()), Some("hi".to_string()));
        assert_eq!(empty_to_none("   ".to_string()), None);
        assert_eq!(empty_to_none(String::new()), None);
    }

    #[test]
    fn required_rejects_blank_with_field_name() {
        assert_eq!(required(" x ".to_string(), "name"), Ok("x".to_string()));
        assert_eq!(
            required("  ".to_string(), "name"),
            Err("name is required".to_string())
        );
    }
}
