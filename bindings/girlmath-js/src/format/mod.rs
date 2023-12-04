use girlmath::format::{format_currency_amount, parse_currency_str};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Format;

#[wasm_bindgen]
impl Format {
    #[wasm_bindgen(js_name = formatCurrencyAmount)]
    pub fn format_currency_amount(currency_code: &str, amount_subunits: i32) -> Result<String, String> {
        format_currency_amount(currency_code, amount_subunits)
    }

    #[wasm_bindgen(js_name = parseCurrencyStr)]
    pub fn parse_currency_str(currency_code: &str, amount: &str) -> Result<i32, String> {
        parse_currency_str(currency_code, amount)
    }
}