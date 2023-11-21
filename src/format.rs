use rusty_money::{Money, iso};
use rust_decimal::{Decimal, prelude::ToPrimitive};

/// Parsing currency strings, and eventually formatting float and integer amounts as currency strings

/// Parses a currency amount represented as a string into a floating-point number.
///
/// # Arguments
///
/// * `currency_code` - String slice containing the ISO 4217 currency code.
/// * `amount` - String slice containing the amount as a string. May contain the currency symbol and thousand separating commas.
///
/// # Returns
///
/// Returns a `Result` with the parsed amount as a `f64` on success or an error message as a `String` on failure.
///
/// # Errors
///
/// Returns an error if the currency code is not supported, the amount is not a valid decimal, or conversion to `f64` fails.
///
/// # Examples
///
/// ```
/// use girlmath::format::parse_currency_amount;
/// let result = parse_currency_amount("USD", "$1,000.50");
/// assert_eq!(result, Ok(1000.50));
/// ```
pub fn parse_currency_amount(currency_code: &str, amount: &str) -> Result<f64, String> {
    let amount_decimal = currency_amount_string_to_decimal(currency_code, amount)?;

    match amount_decimal.to_f64() {
        Some(val) => return Ok(val),
        None => return Err("amount could not be converted to f64".to_string()),
    }
}

// TODO(diehuxx): I want to expose this function to other files in the crate, but not externally from the crate. How do?
/// Converts a currency amount represented as a string to a `Decimal` type.
///
/// # Arguments
///
/// * `currency_code` - The currency code.
/// * `amount` - The amount as a string.
///
/// # Returns
///
/// Returns a `Result` with the amount represented as a `Decimal` on success or an error message as a `String` on failure.
///
/// # Errors
///
/// Returns an error if the currency code is not supported, the amount is not a valid decimal, or other conversion errors occur.
///
pub fn currency_amount_string_to_decimal(currency_code: &str, amount: &str) -> Result<Decimal, String> {
    let currency = match iso::find(currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };

    let amount_without_currency_symbol = amount.replace(&currency.symbol, "");
    let amount_without_currency_symbol_or_commas = amount_without_currency_symbol.replace(",", "");
    let amount_money = match Money::from_str(&amount_without_currency_symbol_or_commas, currency) {
        Ok(val) => val,
        Err(_) => return Err("amount is not a decimal".to_string()),
    };

    return Ok(*amount_money.amount());
}
