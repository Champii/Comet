#[macro_export]
macro_rules! comet {
    ($($e:tt)+) => {
        generate_proto! {}

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen(start)]
        pub fn main() {
            comet::run($($e)+);

            spawn_local(async { main_async().await });
        }

        #[cfg(target_arch = "wasm32")]
        pub async fn main_async() {
            spawn_local(start_socket());
        }

        #[cfg(target_arch = "wasm32")]
        pub async fn start_socket() {
            use comet::prelude::futures::StreamExt;

            let addr = "ws://localhost:8080/ws".to_string();

            let mut socket = Socket::connect(addr).await;

            socket.send(Proto::Todo(TodoProto::New(Todo{
                completed: false,
                title: "hello".to_string(),
            }))).await;

            let mut rx = socket.take_receiver().unwrap();

            while let Some(packet) = rx.next().await {
                // info!("{:#?}", packet);

                comet::console_log!("{:#?}", packet);
            }
            // start_game(socket).await;
        }


        #[cfg(not(target_arch = "wasm32"))]
        pub async fn main() {
            comet::server::server::run::<Proto>().await;
        }
    }
}
