mod console_log;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct TestStruct {

}

#[wasm_bindgen]
impl TestStruct {

    #[wasm_bindgen]
    pub fn foo() {
        console_log::log_str(&"Foo");
    }

    #[wasm_bindgen]
    pub fn bar(&self) {
        console_log::log_str(&"Bar");
    }

}

#[wasm_bindgen]
pub fn create() -> TestStruct {
    TestStruct {}
}