use axum::extract::ws::{Message, WebSocket};
use futures::SinkExt;
use tokio::sync::RwLock;

use futures::stream::SplitSink;
use std::sync::Arc;

use crate::core::prelude::Proto;

use super::universe::Universe;

#[derive(Clone, Debug)]
pub struct Client {
    out: Arc<RwLock<SplitSink<WebSocket, Message>>>,
    session_id: usize,
    universe: Universe,
    // hash: String,
    // db: DatabaseConnection,
}

impl Client {
    pub fn new(out: Arc<RwLock<SplitSink<WebSocket, Message>>>, universe: Universe) -> Self {
        Self {
            out,
            session_id: 0,
            universe,
            // hash: "".to_string(),
            // db: DatabaseConnection::new(),
        }
    }

    pub fn set_session_id(&mut self, session_id: usize) {
        self.session_id = session_id;
    }

    pub fn handle_msg<P: Proto>(&self, msg: Vec<u8>) {
        let proto = P::from_bytes(&msg);

        proto.dispatch();
    }

    pub async fn send<P: Proto>(&self, proto: P) {
        self.out
            .write()
            .await
            .send(Message::Binary(proto.to_bytes()))
            .await
            .unwrap();
    }
}
