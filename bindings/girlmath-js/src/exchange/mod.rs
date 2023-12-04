use girlmath::exchange::{convert_currency_as_int, convert_currency_as_str};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Exchange;

#[wasm_bindgen]
impl Exchange {
    #[wasm_bindgen(js_name = convertCurrencyAsInt)]
    pub fn convert_currency_as_int_wasm(
        payin_currency_code: &str,
        payout_currency_code: &str,
        payin_amount_subunits: i32,
        exchange_rate: f32,
    ) -> Result<i32, String> {
       convert_currency_as_int(payin_currency_code, payout_currency_code, payin_amount_subunits, exchange_rate)
    }

    #[wasm_bindgen(js_name = convertCurrencyAsStr)]
    pub fn convert_currency_as_str_wasm(
      payin_currency_code: &str,
      payout_currency_code: &str,
      payin_amount: &str,
      exchange_rate: f32,
    ) -> Result<String, String> {
      convert_currency_as_str(payin_currency_code, payout_currency_code, payin_amount, exchange_rate)
    }
}
