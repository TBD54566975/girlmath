// TODO: remove when `per-package-target` feature will be stable
#![cfg(target_arch = "wasm32")]

use girlmath::format::format_currency_amount;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format_currency_amount_wasm(currency_code: &str, amount_subunits: i32) -> Result<String, String> {
    format_currency_amount(currency_code, amount_subunits)
}

/// Run some stuff when the Wasm module is instantiated.
///
/// Right now, it does the following:
///
/// * Redirect Rust panics to JavaScript console.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}