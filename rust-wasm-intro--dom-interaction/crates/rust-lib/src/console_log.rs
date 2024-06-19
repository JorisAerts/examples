use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(variadic, js_namespace = console, js_name = log)]
    fn console_log_num(arr: &[i32]);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log_str(s: &str);
}

pub fn log_num(s: i32) {
    console_log_num(&[s]);
}

pub fn log_num2(s: i32, s2: i32) {
    console_log_num(&[s, s2]);
}

pub fn log_str(s: &str) {
    console_log_str(s);
}
