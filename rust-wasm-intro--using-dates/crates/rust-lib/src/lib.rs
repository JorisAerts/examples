use std::time::SystemTime;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn now() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}