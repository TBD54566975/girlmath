use rusty_money::{Money, iso};
use rust_decimal::{Decimal, prelude::{FromPrimitive, ToPrimitive}};
use std::str::FromStr;

/// Functions for converting currency amounts between unit and subunit representations

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
pub fn from_subunits(currency_code: &str, amount_subunits: &str) -> Result<String, String> {
  let currency = match iso::find(currency_code) {
      Some(val) => val,
      None => return Err("currency code not supported".to_string()),
  };

  let decimal_amount = match Decimal::from_str(amount_subunits) {
      Ok(val) => val,
      Err(_) => return Err("amount_subunits is not a decimal".to_string()),
  };

  let multiplier = Decimal::from(10u32.pow(currency.exponent));

  let major_units = decimal_amount / multiplier;

  return Ok(major_units.to_string());
}

/// Converts an exchange rate from a ratio of currency units to a ratio of subunits.
///
/// # Arguments
///
/// * `numerator_currency_code` - The ISO 4217 currency code of the numerator. For an exchange rate of $30,000 / BTC, USD is the numerator.
/// * `denominator_currency_code` - The ISO 4217 currency code of the denominator. E.g. For an exchange rate of $30,000 / BTC, BTC is the denominator.
/// * `exchange_rate` - The exchange rate as a ratio of numerator currency units to denominator currency units.
///
/// # Returns
///
/// Returns a `Result` with the exchange rate as a ratio of subunits as a `f64` on success or an error message as a `String` on failure.
///
/// # Errors
///
/// Returns an error if currency codes are not supported, the exchange rate is not a decimal, or other conversion errors occur.
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

/// Converts an exchange rate from a ratio of currency subunits to a ratio of units.
///
/// # Arguments
///
/// * `numerator_currency_code` - The ISO 4217 currency code of the numerator. For an exchange rate of $30,000 / BTC, USD is the numerator.
/// * `denominator_currency_code` - The ISO 4217 currency code of the denominator. E.g. For an exchange rate of $30,000 / BTC, BTC is the denominator.
/// * `exchange_rate` - The exchange rate as a ratio of numerator currency subunits to denominator currency subunits.
///
/// # Returns
///
/// Returns a `Result` with the exchange rate as a `f64` on success or an error message as a `String` on failure.
///
/// # Errors
///
/// Returns an error if currency codes are not supported, the exchange rate is not a decimal, or other conversion errors occur.
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