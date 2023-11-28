use rust_decimal::{prelude::ToPrimitive, Decimal};
use rusty_money::{iso, Money};

/// Parse a string currency amount to an integer amount of minor currency units
pub fn parse_currency_str(currency_code: &str, amount: &str) -> Result<i64, String> {
    let currency =
        iso::find(currency_code).ok_or_else(|| "currency code not supported".to_string())?;

    let amount_without_currency_symbol = amount.replace(&currency.symbol, "");
    let amount_money = Money::from_str(&amount_without_currency_symbol, currency)
        .map_err(|_| "amount is not a decimal".to_string())?;

    let multiplier: Decimal = Decimal::from(10u32.pow(currency.exponent));
    let amount_subunits_decimal = multiplier * amount_money.amount();
    let amount_subunits = amount_subunits_decimal
        .to_i64()
        .ok_or_else(|| "amount could not be converted to subunits".to_string())?;

    return Ok(amount_subunits);
}

/// Format a currency string given an integer amount of minor currency units
/// The currenct string will contain the currency symbol and use decimal and thousand separators
/// used in the region where the currency is used. E.g. Euro amounts will be formatted as "€20,75".
pub fn format_currency_amount(currency_code: &str, amount_subunits: i64) -> Result<String, String> {
    let currency = match iso::find(currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };

    let amount_money = Money::from_minor(amount_subunits, currency);

    return Ok(amount_money.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_format() {
        let parse_result = parse_currency_str("USD", "$10.50");
        assert_eq!(parse_result, Ok(1050));
        let format_result = format_currency_amount("USD", parse_result.unwrap());
        assert_eq!(format_result, Ok("$10.50".to_string()));

        let parse_result = parse_currency_str("EUR", "€20,75");
        assert_eq!(parse_result, Ok(2075));
        let format_result = format_currency_amount("EUR", parse_result.unwrap());
        assert_eq!(format_result, Ok("€20,75".to_string()));
    }
}
