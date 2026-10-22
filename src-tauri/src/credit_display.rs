//! Display only; never use rounded strings for balances or accounting.
pub(crate) fn amount(raw: &str) -> String {
    let format = || -> Option<String> {
        let mut parts = raw.split('.');
        let whole = parts.next()?;
        let frac = parts.next().unwrap_or("");
        if parts.next().is_some()
            || whole.is_empty()
            || raw.ends_with('.')
            || !whole
                .bytes()
                .chain(frac.bytes())
                .all(|b| b.is_ascii_digit())
        {
            return None;
        }
        let fraction = format!("{frac:0<3}");
        let mut cents = whole
            .parse::<u128>()
            .ok()?
            .checked_mul(100)?
            .checked_add(fraction[..2].parse::<u128>().ok()?)?;
        if fraction.as_bytes()[2] >= b'5' {
            cents = cents.checked_add(1)?;
        }
        Some(format!("{}.{:02}", cents / 100, cents % 100))
    };
    format().unwrap_or_else(|| "—".into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rounds_exact_decimal_strings_to_two_places() {
        for (raw, shown) in [
            ("1234.56789", "1234.57"),
            ("1.005", "1.01"),
            ("999.9999", "1000.00"),
            ("0", "0.00"),
            ("0.1", "0.10"),
            ("0.0001", "0.00"),
            ("9007199254740993.125", "9007199254740993.13"),
        ] {
            assert_eq!(amount(raw), shown);
        }
        for raw in ["", "-1", "NaN", "1e3", "12.bad"] {
            assert_eq!(amount(raw), "—");
        }
    }
}
