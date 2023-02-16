use std::fmt::Debug;
use std::sync::Arc;

use std::net::SocketAddr;

use tokio::sync::RwLock;

use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        ConnectInfo,
    },
    response::Response,
    routing::get,
    Extension, Router,
};
use axum_extra::routing::SpaRouter;

use crate::core::prelude::ProtoTrait;

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::{client::Client, universe::Universe};

use futures::stream::StreamExt;
use log::*;

async fn handler<P: ProtoTrait + Send + 'static + Serialize + DeserializeOwned + Debug>(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(universe): Extension<Universe>,
) -> Response
where
    <P as ProtoTrait>::Client: Send,
    P: ProtoTrait<Client = Client>,
{
    debug!("New connection from {}", addr);
    ws.on_upgrade(move |socket| handle_socket::<P>(socket, universe, addr))
}

async fn handle_socket<P: ProtoTrait + Send + 'static + Serialize + DeserializeOwned + Debug>(
    socket: WebSocket,
    universe: Universe,
    addr: SocketAddr,
) where
    <P as ProtoTrait>::Client: Send,
    P: ProtoTrait<Client = Client>,
{
    let (tx, mut rx) = socket.split();

    let tx = Arc::new(RwLock::new(tx));

    let session_id = universe
        .write()
        .await
        .new_client(Client::new(tx.clone(), universe.clone()));

    debug!("Session id: {} for {}", session_id, addr);

    while let Some(msg) = rx.next().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            break;
        };

        let client = universe.read().await.get_client(session_id);

        client.handle_msg::<P>(msg.into()).await;
    }

    debug!("{}: Client disconnected", session_id);

    // client disconnected
    universe.write().await.remove_client(session_id).await;
}

pub async fn run<P: ProtoTrait + Send + 'static + Serialize + DeserializeOwned + Debug>()
where
    <P as ProtoTrait>::Client: Send,
    P: ProtoTrait<Client = Client>,
{
    let app = Router::new()
        .route("/ws", get(handler::<P>))
        .layer(Extension(crate::UNIVERSE.clone()))
        .merge(SpaRouter::new("/assets", "dist"));

    let addr = "0.0.0.0:8080";

    info!("Listening on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
