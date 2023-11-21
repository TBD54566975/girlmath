
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::format::currency_amount_string_to_decimal;
use crate::subunits::{to_subunits, from_subunits, exchange_rate_from_subunits};

/// Converts a currency amount from one currency to another, using subunit values and an exchange rate as a ratio of subunits.
///
/// # Arguments
///
/// * `payin_currency_code` - The ISO 4217 currency code of the amount being converted.
/// * `payout_currency_code` - The ISO 4217 currency code to which the amount is being converted.
/// * `payin_amount_subunits` - The amount in subunits of the payin currency.
/// * `exchange_rate_subunits` - The exchange rate in subunits (e.g., Ratio of cents to satoshis).
///
/// # Returns
///
/// Returns a `Result` with the converted amount in subunits as a string on success or an error message as a `String` on failure.
///
/// # Errors
///
/// Returns an error if translation from subunits fails, conversion to payout currency fails, or translation back to subunits fails.
///
/// # Examples
///
/// ```
/// // Example code demonstrating how to convert currency amounts using subunits and an exchange rate.
/// use girlmath::exchange::convert_currency_subunits;
/// let result = convert_currency_subunits("USD", "CAD", "1000", 1.20);
/// assert_eq!(result, Ok("1200".to_string()));
/// ```
pub fn convert_currency_subunits(
  payin_currency_code: &str,
  payout_currency_code: &str,
  payin_amount_subunits: &str,
  exchange_rate_subunits: f64,
) -> Result<String, String> {
  // Translate from subunits
  let payin_amount = from_subunits(payin_currency_code, payin_amount_subunits)?;
  let exchange_rate = exchange_rate_from_subunits(payout_currency_code, payin_currency_code, exchange_rate_subunits)?;

  // Convert to payout currency
  let payout_amount = convert_currency(payin_currency_code, &payin_amount, exchange_rate)?;

  // Translate back to subunits
  let payout_amount_subunits = to_subunits(payout_currency_code, &payout_amount)?;

  return Ok(payout_amount_subunits.to_string());
}


/// Converts a currency amount from one currency to another using a given exchange rate.
///
/// # Arguments
///
/// * `payin_currency_code` - The currency code of the amount being converted.
/// * `payin_amount` - The amount in the payin currency as a string.
/// * `exchange_rate` - The exchange rate from payin to payout currency (e.g., USD to EUR).
///
/// # Returns
///
/// Returns a `Result` with the converted amount in the payout currency as a string on success or an error message as a `String` on failure.
///
/// # Errors
///
/// Returns an error if the amount or exchange rate is not valid, or if the conversion fails.
///
/// # Examples
///
/// ```
/// use girlmath::exchange::convert_currency;
/// let result = convert_currency("USD", "$100.50", 1.20);
/// assert_eq!(result, Ok("120.60".to_string()));
/// ```
pub fn convert_currency(
  payin_currency_code: &str,
  payin_amount: &str,
  exchange_rate: f64,
) -> Result<String, String> {
  let payin_amount_decimal = currency_amount_string_to_decimal(payin_currency_code, payin_amount)?;

  let exchange_rate_decimal = match Decimal::from_f64(exchange_rate) {
      Some(val) => val,
      None => return Err("exchange_rate is not a decimal".to_string()),
  };

  let payout_amount_decimal = payin_amount_decimal * exchange_rate_decimal;

  let formatted_payout_amount = format!("{:.2}", payout_amount_decimal);

  return Ok(formatted_payout_amount.to_string());
}
