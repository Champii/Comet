#[macro_export]
macro_rules! run {
    ($e:expr) => {
        // pub use comet::prelude::log::*;
        pub use comet::prelude::*;

        #[cfg(target_arch = "wasm32")]
        use std::panic;

        #[cfg(not(target_arch = "wasm32"))]
        mod schema;

        #[derive(Clone)]
        pub struct Wrapper<T>(pub T);

        #[async_trait(?Send)]
        impl ToVirtualNode for Wrapper<i32> {
            async fn to_virtual_node(self) -> VirtualNode {
                self.0.to_string().into()
            }
        }

        #[async_trait(?Send)]
        impl ToVirtualNode for Wrapper<i64> {
            async fn to_virtual_node(self) -> VirtualNode {
                self.0.to_string().into()
            }
        }

        #[async_trait(?Send)]
        impl ToVirtualNode for Wrapper<String> {
            async fn to_virtual_node(self) -> VirtualNode {
                self.0.clone().into()
            }
        }

        #[async_trait(?Send)]
        impl ToVirtualNode for Wrapper<&str> {
            async fn to_virtual_node(self) -> VirtualNode {
                self.0.to_string().into()
            }
        }

        #[async_trait(?Send)]
        impl ToVirtualNode for Wrapper<()> {
            async fn to_virtual_node(self) -> VirtualNode {
                "".into()
            }
        }

        #[async_trait(?Send)]
        impl ToVirtualNode for Wrapper<bool> {
            async fn to_virtual_node(self) -> VirtualNode {
                self.0.to_string().into()
            }
        }

        #[async_trait(?Send)]
        impl<T: ToVirtualNode> ToVirtualNode for Wrapper<Vec<T>> {
            async fn to_virtual_node(self) -> VirtualNode {
                let mut elem = VElement::new("div");

                for child in self.0.into_iter() {
                    let child = child.to_virtual_node().await;
                    elem.children.push(child);
                }

                VirtualNode::from(elem)
            }
        }

        #[async_trait(?Send)]
        impl<T> ToVirtualNode for Wrapper<Option<T>>
        where
            Wrapper<T>: ToVirtualNode,
        {
            async fn to_virtual_node(self) -> VirtualNode {
                match self.0 {
                    Some(child) => Wrapper(child).to_virtual_node().await,
                    None => "".to_string().into(),
                }
            }
        }

        /* #[async_trait(?Send)]
        impl ToVirtualNode for Wrapper<VirtualNode> {
            async fn to_virtual_node(self) -> VirtualNode {
                self.0
            }
        } */

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
            comet::prelude::wasm_logger::init(wasm_logger::Config::new(Level::Trace));

            info!("Starting app...");

            spawn_local(async { main_async().await });
        }

        #[cfg(target_arch = "wasm32")]
        pub async fn main_async() {
            let (ready_tx, ready_rx) = comet::prelude::futures::channel::oneshot::channel();

            spawn_local(start_socket(ready_tx));

            ready_rx.await.unwrap();

            let mut app = comet::_run($e).await;

            debug!("First render");

            let mut vdom = app.run().await;

            trace!("App rendered");

            let (tx, mut rx) = tokio::sync::mpsc::channel(1);

            spawn_local(async move {
                while let Some(_) = rx.recv().await {
                    trace!("Rerender");
                    app.update(&mut vdom).await;
                }
            });

            REDRAW_CHANNEL.write().await.replace(tx);
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
            trace!("Redrawing root");
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

            info!("Connecting to {}", addr);

            let mut socket: Socket<Proto> = Socket::connect(addr).await;

            let mut rx = socket.take_receiver().unwrap();

            SOCKET.write().await.replace(socket);

            ready.send(()).unwrap();

            debug!("Socket ready");

            while let Some(msg) = rx.next().await {
                let proto = Proto::from_bytes(&msg.msg);

                debug!("packet {:#?}", proto);

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

            trace!("New db connection: {}", database_url);

            PgConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[tokio::main]
        pub async fn main() {
            pretty_env_logger::init();

            debug!("Staring server");

            comet::server::server::run::<Proto>().await;
        }
    };
}
