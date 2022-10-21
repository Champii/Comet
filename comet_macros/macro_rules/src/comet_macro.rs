#[macro_export]
macro_rules! comet {
    ($($e:tt)+) => {
        #[wasm_bindgen(start)]
        pub fn main() {
            comet::run($($e)+);
        }
    }
}
