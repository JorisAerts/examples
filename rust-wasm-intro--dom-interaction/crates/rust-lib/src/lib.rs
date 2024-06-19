use web_sys::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = createHello)]
pub fn create_hello() -> Result<web_sys::Node, JsValue> {
    let document = web_sys::window().unwrap().document().expect("Document should be present.");
    let body = document.body().unwrap();

    let div = document.create_element("div").unwrap();
    div.set_text_content(Some("Hello World"));

    body.append_child(&div)
}
