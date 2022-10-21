use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn consolelog(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (consolelog(&format_args!($($t)*).to_string()))
}
