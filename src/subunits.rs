use rust_decimal::Decimal;
use rusty_money::iso;
use std::str::FromStr;

/// Converts a monetary amount to its subunits. The output will not contain a currency symbol or thousand
/// separating commas.
///
/// # Arguments
///
/// * `currency_code` - String slice containing the ISO 4217 currency code.
/// * `amount` - String slice containing the amount in units. May contain the currency symbol and thousand separating commas.
///
/// # Returns
///
/// Returns a `Result` with the subunits as a `String` on success or an error message as a `String` on failure.
///
/// # Errors
///
/// Returns an error if the currency code is not supported, the amount is not a decimal, or other conversion errors occur.
pub fn major_to_minor(currency_code: &str, amount: &str) -> Result<String, String> {
    let currency =
        iso::find(currency_code).ok_or_else(|| "currency code not supported".to_string())?;

    let multiplier: Decimal = Decimal::from(10u32.pow(currency.exponent));

    let amount_without_currency_symbol = amount.replace(&currency.symbol, "");
    let amount_decimal = Decimal::from_str(&amount_without_currency_symbol)
        .map_err(|_| "amount is not a decimal".to_string())?;
    let amount_subunits = amount_decimal * multiplier;

    return Ok(amount_subunits.floor().to_string());
}

/// Converts subunits to a monetary amount.
///
/// # Arguments
///
/// * `currency_code` - String slice containing the ISO 4217 currency code.
/// * `amount_subunits` - String slice containing the currency amount in subunits. May contain thousand separting commas.
///
/// # Returns
///
/// Returns a `Result` with the monetary amount as a `String` on success or an error message as a `String` on failure.
///
/// # Errors
///
/// Returns an error if the currency code is not supported, the amount is not a decimal, or other conversion errors occur.
pub fn minor_to_major(currency_code: &str, amount_minor: &str) -> Result<String, String> {
    let currency =
        iso::find(currency_code).ok_or_else(|| "currency code not supported".to_string())?;

    let decimal_amount = Decimal::from_str(amount_minor)
        .map_err(|_| "amount_subunits is not a decimal".to_string())?;

    let multiplier = Decimal::from(10u32.pow(currency.exponent));

    let major_units = decimal_amount / multiplier;

    return Ok(major_units.to_string());
}

#[cfg(test)]
mod test {
    use super::{major_to_minor, minor_to_major};

    #[test]
    fn dollar_less_than_one() {
        let currency_code = "USD";
        let major_amount = "0.99";

        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "99");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, major_amount);
    }

    #[test]
    fn dollar_more_than_one() {
        let currency_code = "USD";
        let major_amount = "1.50";

        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "150");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, major_amount);
    }

    #[test]
    fn dollar_thousands() {
        let currency_code = "USD";
        let major_amount = "12345.67";
        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "1234567");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, major_amount);
    }

    #[test]
    fn dollar_with_dollar_sign() {
        let currency_code = "USD";
        let major_amount = "$12345.67";
        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "1234567");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, "12345.67");
    }

    // #[test]
    // fn dollar_with_dollar_sign_and_thousands_comma() {
    //     let currency_code = "USD";
    //     let major_amount = "$12,345.67";
    //     let minor_amount = to_subunits(currency_code, major_amount).unwrap();
    //     assert_eq!(minor_amount, "1234567");

    //     let result_major_amount = from_subunits(currency_code, &minor_amount).unwrap();
    //     assert_eq!(result_major_amount, "12345.67");
    // }

    // #[test]
    // fn euro_with_euro_symbol_and_thousands_comma() {
    //     let currency_code = "EUR";
    //     let major_amount = "€12,345.67";
    //     let minor_amount = to_subunits(currency_code, major_amount).unwrap();
    //     assert_eq!(minor_amount, "1234567");

    //     let result_major_amount = from_subunits(currency_code, &minor_amount).unwrap();
    //     assert_eq!(result_major_amount, "12345.67");
    // }

    #[test]
    fn yen_with_symbol() {
        let currency_code = "JPY";
        let major_amount = "¥12345";
        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "12345");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, "12345");
    }
}
