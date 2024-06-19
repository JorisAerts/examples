use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = benchmarkWasm)]
pub fn benchmark_wasm(n: i32) -> i32 {
    if n == 1 || n == 2 { return 1; }
    return benchmark_wasm(n - 1) + benchmark_wasm(n - 2);
}