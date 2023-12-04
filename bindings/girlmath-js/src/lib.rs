// TODO: remove when `per-package-target` feature will be stable
#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;

pub mod format;
pub mod subunits;
pub mod exchange;

/// Run some stuff when the Wasm module is instantiated.
///
/// Right now, it does the following:
///
/// * Redirect Rust panics to JavaScript console.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}