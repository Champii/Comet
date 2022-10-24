use std::sync::Arc;

use tokio::sync::RwLock;

use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Extension, Router,
};
use axum_extra::routing::SpaRouter;

use crate::core::prelude::Proto;

use super::{client::Client, universe::Universe};

use futures::stream::StreamExt;

async fn handler<P: Proto + Send + 'static>(
    ws: WebSocketUpgrade,
    Extension(universe): Extension<Universe>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket::<P>(socket, universe))
}

async fn handle_socket<P: Proto + Send + 'static>(socket: WebSocket, universe: Universe) {
    let (tx, mut rx) = socket.split();

    let tx = Arc::new(RwLock::new(tx));

    let session_id = universe
        .write()
        .await
        .new_client(Client::new(tx.clone(), universe.clone()));

    while let Some(msg) = rx.next().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        let client = universe.read().await.get_client(session_id);

        client.handle_msg::<P>(msg.into()).await;
    }
}

pub async fn run<P: Proto + Send + 'static>() {
    let app = Router::new()
        .route("/ws", get(handler::<P>))
        .layer(Extension(Universe::default()))
        .merge(SpaRouter::new("/assets", "dist"));

    let addr = "0.0.0.0:8080";

    println!(" -> Listening on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
