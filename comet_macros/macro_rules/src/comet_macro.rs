#[macro_export]
macro_rules! comet {
    ($($e:tt)+) => {
        generate_proto! {}

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen(start)]
        pub fn main() {
            comet::run($($e)+);
        }

        #[cfg(not(target_arch = "wasm32"))]
        pub async fn main() {
            comet::server::server::run::<Proto>().await;
        }
    }
}
