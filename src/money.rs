//! US-dollar formatting and deposit helpers shared across Sigma storefront
//! services.

/// Deposit required to reserve a build (50% of the list price).
#[must_use]
pub fn deposit_cents_for_price(price_cents: u64) -> u64 {
    price_cents / 2
}

/// Render a cents amount as a US dollar string, e.g. `175000` -> `$1,750.00`.
#[must_use]
pub fn format_price_cents(cents: u64) -> String {
    format!("${}.{:02}", group_thousands(cents / 100), cents % 100)
}

/// Render a whole-dollar amount with thousands separators
/// (e.g. `175000` -> `175,000`).
#[must_use]
pub fn group_thousands(dollars: u64) -> String {
    let digits = dollars.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, ch) in digits.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(ch);
    }
    grouped.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_price_cents_groups_thousands() {
        assert_eq!(format_price_cents(1999), "$19.99");
        assert_eq!(format_price_cents(17_500_000), "$175,000.00");
        assert_eq!(format_price_cents(100), "$1.00");
        assert_eq!(format_price_cents(0), "$0.00");
    }

    #[test]
    fn deposit_is_half_the_price() {
        assert_eq!(deposit_cents_for_price(200_000), 100_000);
        assert_eq!(deposit_cents_for_price(175_000), 87_500);
        assert_eq!(deposit_cents_for_price(1), 0);
    }

    #[test]
    fn group_thousands_separates_every_three_digits() {
        assert_eq!(group_thousands(0), "0");
        assert_eq!(group_thousands(999), "999");
        assert_eq!(group_thousands(1_000), "1,000");
        assert_eq!(group_thousands(1_234_567), "1,234,567");
    }
}
