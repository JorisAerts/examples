use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = getNumber)]
pub fn get_number() -> i32 {
  123
}