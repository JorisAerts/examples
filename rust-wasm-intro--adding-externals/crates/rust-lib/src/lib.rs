mod console_log;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hello() {
    console_log::log_num(1);
    console_log::log_num2(123,456);
}