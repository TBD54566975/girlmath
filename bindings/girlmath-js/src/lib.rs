use girlmath;
use wasm_bindgen::{prelude::*, JsError};

/// Run some stuff when the Wasm module is instantiated.
///
/// Right now, it does the following:
///
/// * Redirect Rust panics to JavaScript console.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

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
#[wasm_bindgen(js_name = majorToMinor)]
pub fn major_to_minor_wasm(amt: &str, currency_code: &str) -> Result<String, JsError> {
    return girlmath::major_to_minor(amt, currency_code).map_err(|e| JsError::new(&e));
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
#[wasm_bindgen(js_name = minorToMajor)]
pub fn minor_to_major_wasm(amt: &str, currency_code: &str) -> Result<String, JsError> {
    return girlmath::minor_to_major(amt, currency_code).map_err(|e| JsError::new(&e));
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
#[wasm_bindgen(js_name = convert)]
pub fn convert_wasm(
    payin_amt: &str,
    _payin_currency: &str,
    payout_currency: &str,
    rate: &str,
) -> Result<String, JsError> {
    return girlmath::convert(payin_amt, _payin_currency, payout_currency, rate)
        .map_err(|e| JsError::new(&e));
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
/// ```js
///
/// const invertedRate = invertRate("40370.46311", "BTC")
/// console.assert("0.00002477058", invertedRate);
/// ```
#[wasm_bindgen(js_name = invertRate)]
pub fn invert_rate(rate: &str, currency_code: &str) -> Result<String, JsError> {
    return girlmath::invert_rate(rate, currency_code).map_err(|e| JsError::new(&e));
}
