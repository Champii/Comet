#[macro_export]
macro_rules! comet {
    ($($e:tt)+) => {
        generate_proto! {}

        #[wasm_bindgen(start)]
        pub fn main() {
            comet::run($($e)+);
        }
    }
}
