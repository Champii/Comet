use axum::extract::ws::{Message, WebSocket};
use futures::SinkExt;
use tokio::sync::RwLock;

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

use futures::stream::SplitSink;
use std::sync::Arc;

use crate::core::prelude::Proto;

use super::universe::Universe;

#[derive(Clone, Debug)]
pub struct Client {
    out: Arc<RwLock<SplitSink<WebSocket, Message>>>,
    session_id: usize,
    // universe: Universe,
    // hash: String,
    // db: DatabaseConnection,
}

impl Client {
    pub fn new(out: Arc<RwLock<SplitSink<WebSocket, Message>>>, _universe: Universe) -> Self {
        Self {
            out,
            session_id: 0,
            // universe,
            // hash: "".to_string(),
            // db: DatabaseConnection::new(),
        }
    }

    pub fn set_session_id(&mut self, session_id: usize) {
        self.session_id = session_id;
    }

    pub async fn handle_msg<P: Proto + Send + Serialize + DeserializeOwned + Debug>(&self, msg: Vec<u8>) {
        let msg = crate::Message::from_bytes(&msg);
        
        let proto = P::from_bytes(&msg.msg);

        let response = proto.dispatch().await;

        if let Some(response) = response {
            let response = response.to_bytes();

            let msg = crate::Message {
                request_id: msg.request_id,
                msg: response,
            };

            let response = msg.to_bytes();

            self.out.write().await.send(Message::Binary(response)).await.unwrap();
        }

    }

    /* pub async fn send<P: Proto + Send + Serialize + DeserializeOwned>(&self, proto: P) {
        let msg = proto.to_bytes();
        let msg = crate::Message {request_id: 0, msg};
        let msg = msg.to_bytes();

        self.out
            .write()
            .await
            .send(Message::Binary(msg))
            .await
            .unwrap();
    } */
}
