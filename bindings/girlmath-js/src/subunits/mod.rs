use girlmath::subunits::{major_to_minor, minor_to_major};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Subunits;

#[wasm_bindgen]
impl Subunits {
    #[wasm_bindgen(js_name = majorToMinor)]
    pub fn major_to_minor_wasm(currency_code: &str, amount: &str) -> Result<String, String> {
        major_to_minor(currency_code, amount)
    }

    #[wasm_bindgen(js_name = minorToMajor)]
    pub fn minor_to_major_wasm(currency_code: &str, amount_minor: &str) -> Result<String, String> {
      minor_to_major(currency_code, amount_minor)
    }
}
