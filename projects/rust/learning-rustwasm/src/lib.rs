use wasm_bindgen::prelude::*;
use web_sys::console;

mod utils;
pub mod util;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// 问候.
#[wasm_bindgen]
pub fn log(name: &str) {
    console::log_1(&format!("HELLO, {}", name).into());
}

#[cfg(test)]
mod tests {
    #[test]
    fn _01() {}
}