use rusty_money::{Money, iso};
use rust_decimal::{Decimal, prelude::{FromPrimitive, ToPrimitive}};
use std::str::FromStr;

// functions for converting between number and string representations
pub fn parse_currency_amount(currency_code: &str, amount_major: &str) -> Result<f64, String> {
    let amount_decimal = match currency_amount_string_to_decimal(currency_code, amount_major) {
        Ok(val) => val,
        Err(e) => return Err(e)
,    };

    match amount_decimal.to_f64() {
        Some(val) => return Ok(val),
        None => return Err("amount could not be converted to f64".to_string()),
    }
}

fn currency_amount_string_to_decimal(currency_code: &str, amount_major: &str) -> Result<Decimal, String> {
    let currency = match iso::find(currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };

    let amount_without_currency_symbol = amount_major.replace(&currency.symbol, "");
    let amount_money = match Money::from_str(&amount_without_currency_symbol, currency) {
        Ok(val) => val,
        Err(_) => return Err("amount is not a decimal".to_string()),
    };

    return Ok(*amount_money.amount());
}

// functions for converting between major and minor string representations
pub fn major_to_minor(currency_code: &str, amount_major: &str) -> Result<String, String> {
    let currency = match iso::find(currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };

    let multiplier = Decimal::from(10u32.pow(currency.exponent));

    let amount_without_currency_symbol = amount_major.replace(&currency.symbol, "");
    let amount_money = match Money::from_str(&amount_without_currency_symbol, currency) {
        Ok(val) => val,
        Err(_) => return Err("amount is not a decimal".to_string()),
    };
    let amount_decimal = amount_money.amount();

    let minor_amount = amount_decimal * multiplier;

    return Ok(minor_amount.floor().to_string());
}

pub fn minor_to_major(currency_code: &str, amount_minor: &str) -> Result<String, String> {
    let currency = match iso::find(currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };

    let decimal_amount = match Decimal::from_str(amount_minor) {
        Ok(val) => val,
        Err(_) => return Err("amount is not a decimal".to_string()),
    };

    let multiplier = Decimal::from(10u32.pow(currency.exponent));

    let major_units = decimal_amount / multiplier;

    return Ok(major_units.to_string());
}

pub fn exchange_rate_major_to_minor(
    numerator_currency_code: &str,
    denominator_currency_code: &str,
    exchange_rate_major: f64,
) -> Result<f64, String> {
    // Get numerator currency's ratio of minor units to major units
    let numerator_currency = match iso::find(numerator_currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };
    let numerator_minor_per_major = Decimal::from(10u32.pow(numerator_currency.exponent));

    // Get denominator currency's ratio of minor units to major units
    let denominator_currency = match iso::find(denominator_currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };
    let denominator_minor_per_major = Decimal::from(10u32.pow(denominator_currency.exponent));

    let exchange_rate_major_decimal = match Decimal::from_f64(exchange_rate_major) {
        Some(val) => val,
        None => return Err("exchange_rate is not a decimal".to_string()),
    };

    // exchange_rate_minor = (numerator_minor / denominator_minor)
    //                     = (numerator_major / denominator_major) * (numerator_minor / numerator_major) * (denominator_major / denominator_minor)
    let exchange_rate_minor = exchange_rate_major_decimal * numerator_minor_per_major / denominator_minor_per_major;

    match exchange_rate_minor.to_f64() {
        Some(val) => return Ok(val),
        None => return Err("exchange rate could not be converted to f64".to_string()),
    }
}

pub fn exchange_rate_minor_to_major(
    numerator_currency_code: &str,
    denominator_currency_code: &str,
    exchange_rate_minor: f64,
) -> Result<f64, String> {
    // Get numerator currency's ratio of minor units to major units
    let numerator_currency = match iso::find(numerator_currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };
    let numerator_minor_per_major = Decimal::from(10u32.pow(numerator_currency.exponent));

    // Get denominator currency's ratio of minor units to major units
    let denominator_currency = match iso::find(denominator_currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };
    let denominator_minor_per_major = Decimal::from(10u32.pow(denominator_currency.exponent));

    let exchange_rate_minor_decimal = match Decimal::from_f64(exchange_rate_minor) {
        Some(val) => val,
        None => return Err("exchange_rate is not a decimal".to_string()),
    };

    // exchange_rate_minor = (numerator_major / denominator_major)
    //                     = (numerator_minor/ denominator_minor) * (numerator_major / numerator_minor) * (denominator_minor / denominator_major)
    let exchange_rate_major = exchange_rate_minor_decimal * numerator_minor_per_major / denominator_minor_per_major;

    match exchange_rate_major.to_f64() {
        Some(val) => return Ok(val),
        None => return Err("exchange rate could not be converted to f64".to_string()),
    };
}

// functions for converting amounts between different currencies
pub fn convert_currency_minor(
    payin_currency_code: &str,
    payout_currency_code: &str,
    payin_amount_minor: &str,
    payout_per_payin_minor: f64,
) -> Result<String, String> {
    let payin_amount_major = match minor_to_major(payin_currency_code, payin_amount_minor) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    let payout_per_payin_major = match exchange_rate_minor_to_major(payout_currency_code, payin_currency_code, payout_per_payin_minor) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    return convert_currency_major(payin_currency_code, &payin_amount_major, payout_per_payin_major);
}

pub fn convert_currency_major(
    payin_currency_code: &str,
    payin_amount_major: &str,
    payout_per_payin_major: f64,
) -> Result<String, String> {
    let payin_amount_decimal = match currency_amount_string_to_decimal(payin_currency_code, payin_amount_major) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    let payout_per_payin_decimal = match Decimal::from_f64(payout_per_payin_major) {
        Some(val) => val,
        None => return Err("payout_per_payin_major is not a decimal".to_string()),
    };

    let payout_amount_decimal = payin_amount_decimal * payout_per_payin_decimal;

    return Ok(payout_amount_decimal.to_string());
}

/// Alias of `convert_currency_major`
pub fn convert_currency(
    payin_currency_code: &str,
    payin_amount_major_units: &str,
    exchange_rate_major: f64,
) -> Result<String, String> {
    return convert_currency_major(
        payin_currency_code,
        payin_amount_major_units,
        exchange_rate_major
    );
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

    #[test]
    fn dollar_with_dollar_sign_and_thousands_comma() {
        let currency_code = "USD";
        let major_amount = "$12,345.67";
        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "1234567");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, "12345.67");
    }

    #[test]
    fn euro_with_euro_symbol_and_thousands_comma() {
        let currency_code = "EUR";
        let major_amount = "€12.345,67";
        let minor_amount = major_to_minor(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "1234567");

        let result_major_amount = minor_to_major(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, "12345.67");
    }

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