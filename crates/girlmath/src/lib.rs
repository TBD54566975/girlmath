use std::str::FromStr;

use rust_decimal::Decimal;
use rusty_money::crypto;
use rusty_money::iso;

/// Converts a major currency unit (like dollars) to a minor currency unit (like cents).
///
/// # Arguments
/// * `amt` - A string slice that holds the amount in major currency units.
/// * `currency_code` - The ISO or cryptocurrency code as a string slice.
///
/// # Returns
/// A `Result<String, String>` which is Ok with the amount in minor units
/// or an Err with an error message.
///
/// # Examples
/// ```
/// use girlmath::major_to_minor;
///
/// let cents = major_to_minor("10.50", "USD").unwrap();
/// assert_eq!("1050", cents);
/// ```
pub fn major_to_minor(amt: &str, currency_code: &str) -> Result<String, String> {
    let sig_digs = iso::find(currency_code)
        .map(|c| c.exponent)
        .or_else(|| crypto::find(currency_code).map(|c| c.exponent))
        .ok_or_else(|| "unsupported currency code".to_string())?;

    let multiplier = Decimal::from(10u32.pow(sig_digs));
    let amt_dec = Decimal::from_str(amt).map_err(|_| "invalid amount".to_string())?;

    let minor_amt = (multiplier * amt_dec).trunc();

    return Ok(minor_amt.to_string());
}

/// Converts a minor currency unit (like cents) to a major currency unit (like dollars).
///
/// # Arguments
/// * `amt` - A string slice that holds the amount in minor currency units.
/// * `currency_code` - The ISO or cryptocurrency code as a string slice.
///
/// # Returns
/// A `Result<String, String>` which is Ok with the amount in major units
/// or an Err with an error message.
///
/// # Examples
/// ```
/// use girlmath::minor_to_major;
///
/// let dollars = minor_to_major("1050", "USD").unwrap();
/// assert_eq!("10.50", dollars);
/// ```
pub fn minor_to_major(amt: &str, currency_code: &str) -> Result<String, String> {
    let sig_digs = iso::find(currency_code)
        .map(|c| c.exponent)
        .or_else(|| crypto::find(currency_code).map(|c| c.exponent))
        .ok_or_else(|| "unsupported currency code".to_string())?;

    let multiplier = Decimal::from(10u32.pow(sig_digs));
    let amt_dec = Decimal::from_str(amt).map_err(|_| "invalid amount".to_string())?;

    let major_amt = (amt_dec / multiplier).trunc_with_scale(sig_digs);

    return Ok(major_amt.to_string());
}

/// Converts an amount from one currency to another using a given exchange rate.
///
/// # Arguments
/// * `payin_amt` - The amount in the pay-in currency as a string slice.
/// * `payin_currency` - The pay-in currency code as a string slice (currently unused).
/// * `payout_currency` - The pay-out currency code as a string slice.
/// * `rate` - The exchange rate as a string slice.
///
/// # Returns
/// A `Result<String, String>` which is Ok with the converted amount
/// or an Err with an error message.
///
/// # Examples
/// ```
/// use girlmath::convert;
///
/// let payout_amt = convert("3", "USD", "GHS", "12.10").unwrap();
/// assert_eq!("36.30", payout_amt);
/// ```
pub fn convert(
    payin_amt: &str,
    _payin_currency: &str,
    payout_currency: &str,
    rate: &str,
) -> Result<String, String> {
    let sig_digs = iso::find(payout_currency)
        .map(|c| c.exponent)
        .or_else(|| crypto::find(payout_currency).map(|c| c.exponent))
        .ok_or_else(|| "unsupported currency code".to_string())?;

    let payin_dec =
        Decimal::from_str(payin_amt).map_err(|_| "Invalid payin amount.".to_string())?;

    let rate_dec = Decimal::from_str(rate).map_err(|_| "Invalid rate".to_string())?;
    let converted_amt = (payin_dec * rate_dec).trunc_with_scale(sig_digs);

    return Ok(converted_amt.to_string());
}

/// Inverts an exchange rate.
///
/// # Arguments
/// * `rate` - The exchange rate as a string slice.
/// * `currency_code` - The currency code for which the rate is to be inverted.
///
/// # Returns
/// A `Result<String, String>` which is Ok with the inverted rate as a string
/// or an Err with an error message.
///
/// # Examples
/// ```
/// use girlmath::invert_rate;
///
/// let inverted_rate = invert_rate("40370.46311", "BTC").unwrap();
/// assert_eq!("0.00002477058", inverted_rate);
/// ```
pub fn invert_rate(rate: &str, currency_code: &str) -> Result<String, String> {
    let sig_digs = iso::find(currency_code)
        .map(|c| c.exponent)
        .or_else(|| crypto::find(currency_code).map(|c| c.exponent))
        .ok_or_else(|| "unsupported currency code".to_string())?;

    let rate_dec = Decimal::from_str(rate).map_err(|_| "Invalid rate".to_string())?;
    let inverted_rate = (Decimal::ONE / rate_dec).trunc_with_scale(sig_digs + 3);

    return Ok(inverted_rate.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_to_minor() {
        let cents = major_to_minor("10.50", "USD").unwrap();
        assert_eq!("1050", cents);

        let sats = major_to_minor("0.00002568", "BTC").unwrap();
        assert_eq!("2568", sats);
    }

    #[test]
    fn test_minor_to_major() {
        let dollars = minor_to_major("1050", "USD").unwrap();
        assert_eq!("10.50", dollars);

        let btc = minor_to_major("2568", "BTC").unwrap();
        assert_eq!("0.00002568", btc);
    }

    #[test]
    fn test_invert_rate() {
        let btc_usd_exchange_rate = "40370.46311";
        let usd_btc_exchange_rate = invert_rate(btc_usd_exchange_rate, "BTC").unwrap();

        assert_eq!("0.00002477058", usd_btc_exchange_rate);

        let btc_usd_exchange_rate_again = invert_rate(&usd_btc_exchange_rate, "USD").unwrap();

        let btc_usd_f64: f64 = btc_usd_exchange_rate.parse().unwrap();
        let btc_usd_again_f64: f64 = btc_usd_exchange_rate_again.parse().unwrap();

        assert!(
            (btc_usd_f64 - btc_usd_again_f64).abs() <= 0.01,
            "Values are not within the specified tolerance"
        );
    }

    #[test]
    fn test_convert() {
        let usd_ghs_exchange_rate = "12.10";
        let payout_amt = convert("3", "USD", "GHS", usd_ghs_exchange_rate).unwrap();

        assert_eq!("36.30", payout_amt)
    }
}
