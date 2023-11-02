use iso_currency::Currency;
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn major_to_minor(currency_code: &str, amount: &str) -> Result<String, String> {
    let currency = match Currency::from_str(currency_code) {
        Ok(val) => val,
        Err(_) => return Err("currency code not supported".to_string()),
    };

    let decimal_amount = match Decimal::from_str(amount) {
        Ok(val) => val,
        Err(_) => return Err("amount is not a decimal".to_string()),
    };

    let multiplier = match currency.exponent() {
        Some(significant_digits) => Decimal::from(10u32.pow(significant_digits.into())),
        None => Decimal::from(1), // some currencies don't have significant digits (e.g. JPY)
    };

    let minor_units = decimal_amount * multiplier;

    return Ok(minor_units.to_string());
}
