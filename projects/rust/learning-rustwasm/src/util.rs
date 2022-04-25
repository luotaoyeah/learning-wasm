use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct Util {}

#[wasm_bindgen]
impl Util {
    pub fn fn01() {
        console::log_1(&("000".into()));
    }
}