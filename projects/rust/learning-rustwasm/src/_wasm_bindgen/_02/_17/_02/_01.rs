use wasm_bindgen::prelude::*;

/// 某人.
#[wasm_bindgen]
struct Person {
    /// 年龄.
    pub age: u32,
}

#[wasm_bindgen]
impl Person {
    /// 构造某人.
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32) -> Self {
        Person { age: x }
    }
}