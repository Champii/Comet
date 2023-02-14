#[macro_export]
macro_rules! run {
    ($e:expr) => {
        pub use comet::prelude::*;

        #[cfg(target_arch = "wasm32")]
        use std::panic;

        mod schema;

        #[derive(Clone)]
        pub struct Wrapper<T>(pub T);

        impl From<Wrapper<i32>> for VirtualNode {
            fn from(wrapper: Wrapper<i32>) -> VirtualNode {
                wrapper.0.to_string().into()
            }
        }

        impl From<Wrapper<&str>> for VirtualNode {
            fn from(wrapper: Wrapper<&str>) -> VirtualNode {
                wrapper.0.into()
            }
        }

        impl From<Wrapper<String>> for VirtualNode {
            fn from(wrapper: Wrapper<String>) -> VirtualNode {
                wrapper.0.into()
            }
        }

        impl From<Wrapper<()>> for VirtualNode {
            fn from(wrapper: Wrapper<()>) -> VirtualNode {
                "".into()
            }
        }

        impl<T: Into<VirtualNode>> From<Wrapper<Vec<T>>> for VirtualNode {
            fn from(wrapper: Wrapper<Vec<T>>) -> VirtualNode {
                let mut elem = VElement::new("div");
                elem.children = wrapper.0.into_iter().map(|child| child.into()).collect();

                VirtualNode::from(elem)
            }
        }

        generate_rpc_proto! {}
        generate_proto! {}

        #[cfg(not(target_arch = "wasm32"))]
        generate_migrations! {}

        #[cfg(target_arch = "wasm32")]
        generate_cache! {}

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

            let mut app = comet::_run($e).await;

            let mut vdom = app.run().await;

            let (tx, mut rx) = tokio::sync::mpsc::channel(1);

            spawn_local(async move {
                while let Some(_) = rx.recv().await {
                    app.update(&mut vdom).await;
                }
            });

            REDRAW_CHANNEL.write().await.replace(tx);

            println!("HERE");
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
        lazy_static! {
            pub static ref REDRAW_CHANNEL: Arc<RwLock<Option<tokio::sync::mpsc::Sender<()>>>> =
                Arc::new(RwLock::new(None));
        }

        #[cfg(target_arch = "wasm32")]
        pub async fn redraw_root() {
            crate::REDRAW_CHANNEL
                .write()
                .await
                .as_mut()
                .unwrap()
                .send(())
                .await
                .unwrap();
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

                // comet::console_log!("packet {:#?}", proto);

                if let Proto::Event(request_id, events) = proto {
                    CACHE
                        .write()
                        .await
                        .update_for_request_id(request_id, events);

                    redraw_root().await;
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
    };
}
