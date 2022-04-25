use wasm_bindgen::prelude::*;

/// 使用 `js_name` 设置输出给 JS 的名称.
#[wasm_bindgen(js_name = "Person02")]
pub struct Person {
    #[wasm_bindgen(js_name = "firstName")]
    pub first_name: u32,
}

/// 因为 Person 设置了输出名称, 因此它的 impl 需要使用 `js_class` 指定这个输出名称.
#[wasm_bindgen(js_class = "Person02")]
impl Person {
    #[wasm_bindgen(js_name = "fn01")]
    pub fn fn_01() -> bool {
        true
    }
}