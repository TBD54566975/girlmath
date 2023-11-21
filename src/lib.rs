use rusty_money::{Money, iso};
use rust_decimal::{Decimal, prelude::{FromPrimitive, ToPrimitive}};
use std::str::FromStr;

// functions for converting between number and string representations
pub fn parse_currency_amount(currency_code: &str, amount_major: &str) -> Result<f64, String> {
    let amount_decimal = match currency_amount_string_to_decimal(currency_code, amount_major) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

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


// functions for converting between units and subunits string representations
pub fn to_subunits(currency_code: &str, amount: &str) -> Result<String, String> {
    let currency = match iso::find(currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };

    let multiplier = Decimal::from(10u32.pow(currency.exponent));

    let amount_without_currency_symbol = amount.replace(&currency.symbol, "");
    let amount_money = match Money::from_str(&amount_without_currency_symbol, currency) {
        Ok(val) => val,
        Err(_) => return Err("amount is not a decimal".to_string()),
    };
    let amount_decimal = amount_money.amount();

    let amount_subunits = amount_decimal * multiplier;

    return Ok(amount_subunits.floor().to_string());
}

pub fn from_subunits(currency_code: &str, amount_subunits: &str) -> Result<String, String> {
    let currency = match iso::find(currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };

    let decimal_amount = match Decimal::from_str(amount_subunits) {
        Ok(val) => val,
        Err(_) => return Err("amount is not a decimal".to_string()),
    };

    let multiplier = Decimal::from(10u32.pow(currency.exponent));

    let major_units = decimal_amount / multiplier;

    return Ok(major_units.to_string());
}

pub fn exchange_rate_to_subunits(
    numerator_currency_code: &str,
    denominator_currency_code: &str,
    exchange_rate: f64,
) -> Result<f64, String> {
    // Get numerator currency's ratio of subunits to units
    let numerator_currency = match iso::find(numerator_currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };
    let numerator_subunits_per_unit = Decimal::from(10u32.pow(numerator_currency.exponent));

    // Get denominator currency's ratio of minor units to major units
    let denominator_currency = match iso::find(denominator_currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };
    let denominator_subunits_per_unit = Decimal::from(10u32.pow(denominator_currency.exponent));

    let exchange_rate_major_decimal = match Decimal::from_f64(exchange_rate) {
        Some(val) => val,
        None => return Err("exchange_rate is not a decimal".to_string()),
    };

    // exchange_rate_subunits = (numerator_subunits / denominator_subunits)
    //                        = (numerator_units / denominator_units) * (numerator_subunits / numerator_major) * (denominator_major / denominator_subunits)
    let exchange_rate_subunits = exchange_rate_major_decimal * numerator_subunits_per_unit / denominator_subunits_per_unit;

    match exchange_rate_subunits.to_f64() {
        Some(val) => return Ok(val),
        None => return Err("exchange rate could not be converted to f64".to_string()),
    }
}

pub fn exchange_rate_from_subunits(
    numerator_currency_code: &str,
    denominator_currency_code: &str,
    exchange_rate_subunits: f64,
) -> Result<f64, String> {
    // Get numerator currency's ratio of subunits to units
    let numerator_currency = match iso::find(numerator_currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };
    let numerator_subunits_per_unit = Decimal::from(10u32.pow(numerator_currency.exponent));

    // Get denominator currency's ratio of subunits to units
    let denominator_currency = match iso::find(denominator_currency_code) {
        Some(val) => val,
        None => return Err("currency code not supported".to_string()),
    };
    let denominator_subunits_per_unit = Decimal::from(10u32.pow(denominator_currency.exponent));

    let exchange_rate_subunits_decimal = match Decimal::from_f64(exchange_rate_subunits) {
        Some(val) => val,
        None => return Err("exchange_rate is not a decimal".to_string()),
    };

    // exchange_rate = (numerator_units / denominator_units)
    //                     = (numerator_subunits/ denominator_subunits) * (numerator_units / numerator_subunits) * (denominator_subunits / denominator_units)
    let exchange_rate = exchange_rate_subunits_decimal * numerator_subunits_per_unit / denominator_subunits_per_unit;

    match exchange_rate.to_f64() {
        Some(val) => return Ok(val),
        None => return Err("exchange rate could not be converted to f64".to_string()),
    };
}

// functions for converting amounts between different currencies
pub fn convert_currency_as_subunits(
    payin_currency_code: &str,
    payout_currency_code: &str,
    payin_amount_subunits: &str,
    payout_per_payin_subunits: f64,
) -> Result<String, String> {
    let payin_amount_major = match from_subunits(payin_currency_code, payin_amount_subunits) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    let payout_per_payin_major = match exchange_rate_from_subunits(payout_currency_code, payin_currency_code, payout_per_payin_subunits) {
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
    use super::{to_subunits, from_subunits};

    #[test]
    fn dollar_less_than_one() {
        let currency_code = "USD";
        let major_amount = "0.99";

        let minor_amount = to_subunits(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "99");

        let result_major_amount = from_subunits(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, major_amount);
    }

    #[test]
    fn dollar_more_than_one() {
        let currency_code = "USD";
        let major_amount = "1.50";

        let minor_amount = to_subunits(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "150");

        let result_major_amount = from_subunits(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, major_amount);
    }

    #[test]
    fn dollar_thousands() {
        let currency_code = "USD";
        let major_amount = "12345.67";
        let minor_amount = to_subunits(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "1234567");

        let result_major_amount = from_subunits(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, major_amount);
    }

    #[test]
    fn dollar_with_dollar_sign() {
        let currency_code = "USD";
        let major_amount = "$12345.67";
        let minor_amount = to_subunits(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "1234567");

        let result_major_amount = from_subunits(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, "12345.67");
    }

    #[test]
    fn dollar_with_dollar_sign_and_thousands_comma() {
        let currency_code = "USD";
        let major_amount = "$12,345.67";
        let minor_amount = to_subunits(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "1234567");

        let result_major_amount = from_subunits(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, "12345.67");
    }

    #[test]
    fn euro_with_euro_symbol_and_thousands_comma() {
        let currency_code = "EUR";
        let major_amount = "€12.345,67";
        let minor_amount = to_subunits(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "1234567");

        let result_major_amount = from_subunits(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, "12345.67");
    }

    #[test]
    fn yen_with_symbol() {
        let currency_code = "JPY";
        let major_amount = "¥12345";
        let minor_amount = to_subunits(currency_code, major_amount).unwrap();
        assert_eq!(minor_amount, "12345");

        let result_major_amount = from_subunits(currency_code, &minor_amount).unwrap();
        assert_eq!(result_major_amount, "12345");
    }
}