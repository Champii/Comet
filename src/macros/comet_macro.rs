#[macro_export]
macro_rules! run {
    ($($e:tt)+) => {

        pub use comet::prelude::*;

        #[cfg(target_arch = "wasm32")]
        use std::panic;

        // #[cfg(not(target_arch = "wasm32"))]
        mod schema;
        // #[cfg(not(target_arch = "wasm32"))]
        // use schema::*;

        generate_rpc_proto! {}
        generate_proto! {}

        #[cfg(not(target_arch = "wasm32"))]
        generate_migrations! {}

        #[cfg(target_arch = "wasm32")]
        generate_cache! {}

        /* pub mod prelude {
            pub use crate::*;
            /* pub use crate::RPCQuery;
            pub use crate::RPCResponse;
            pub use crate::Proto; */
            pub use comet::prelude::*;
        } */
        // pub use crate::prelude::*;

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen(start)]
        pub fn main() {
            panic::set_hook(Box::new(comet::prelude::console_error_panic_hook::hook));

            spawn_local(async { main_async().await });
        }

        #[cfg(target_arch = "wasm32")]
        pub async fn main_async() {
            let (ready_tx, ready_rx) = comet::prelude::futures::channel::oneshot::channel();

            spawn_local(start_socket(ready_tx));

            ready_rx.await.unwrap();

            comet::_run($($e)+).await;
        }

        #[cfg(target_arch = "wasm32")]
        lazy_static! {
            pub static ref SOCKET: Arc<RwLock<Option<Socket<Proto>>>> = Arc::new(RwLock::new(None));
        }

        #[cfg(target_arch = "wasm32")]
        lazy_static! {
            pub static ref CACHE: Arc<RwLock<Cache>> = Arc::new(RwLock::new(Cache::new()));
        }

        #[cfg(target_arch = "wasm32")]
        pub async fn start_socket(ready: comet::prelude::futures::channel::oneshot::Sender<()>) {
            use comet::prelude::futures::StreamExt;

            let addr = "ws://localhost:8080/ws".to_string();

            let mut socket: Socket<Proto> = Socket::connect(addr).await;

            let mut rx = socket.take_receiver().unwrap();

            SOCKET.write().await.replace(socket);

            ready.send(()).unwrap();

            while let Some(msg) = rx.next().await {
                let proto = Proto::from_bytes(&msg.msg);

                comet::console_log!("packet {:#?}", proto);

                if let Proto::Event(request_id, events) = proto {
                    comet::console_log!("cache after event {:#?}", *CACHE.read().await);
                    CACHE.write().await.update_for_request_id(request_id, events);

                    comet::console_log!("cache after event2 {:#?}", *CACHE.read().await);
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        use crate::diesel::pg::PgConnection;

        #[cfg(not(target_arch = "wasm32"))]
        pub fn establish_connection() -> PgConnection {
            use crate::diesel::prelude::*;
            use std::env;

            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

            PgConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[tokio::main]
        pub async fn main() {
            comet::server::server::run::<Proto>().await;
        }
    }
}
