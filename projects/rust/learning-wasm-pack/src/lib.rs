use wasm_bindgen::prelude::*;
use web_sys::console;

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