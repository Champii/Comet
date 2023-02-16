use axum::extract::ws::{Message, WebSocket};
use futures::SinkExt;
use tokio::sync::RwLock;

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use tokio::task::JoinHandle;

use futures::stream::SplitSink;
use std::sync::Arc;

use crate::core::prelude::ProtoTrait;

use super::universe::Universe;

#[derive(Clone, Debug)]
pub struct Client {
    out: Arc<RwLock<SplitSink<WebSocket, Message>>>,
    session_id: usize,
    queries: Arc<RwLock<HashMap<u64, JoinHandle<()>>>>,
    next_request_id: Arc<RwLock<u64>>,
}

impl Client {
    pub fn new(out: Arc<RwLock<SplitSink<WebSocket, Message>>>, _universe: Universe) -> Self {
        Self {
            out,
            session_id: 0,
            queries: Arc::new(RwLock::new(HashMap::new())),
            next_request_id: Arc::new(RwLock::new(0)),
        }
    }

    pub fn set_session_id(&mut self, session_id: usize) {
        self.session_id = session_id;
    }

    pub async fn handle_msg<P: ProtoTrait + Send + Serialize + DeserializeOwned + Debug>(
        &self,
        msg: Vec<u8>,
    ) where
        <P as ProtoTrait>::Client: Send,
        P: ProtoTrait<Client = Self>,
    {
        if msg.is_empty() {
            warn!("{}: Empty message", self.session_id);

            return;
        }

        let msg = crate::Message::from_bytes(&msg);

        trace!("{}: Message: {:?}", self.session_id, msg);

        let proto = P::from_bytes(&msg.msg);

        debug!("{}: Proto: {:?}", self.session_id, proto);

        let response = proto.dispatch(msg.request_id, self.clone()).await;

        if let Some(response) = response {
            debug!("{}: Response: {:?}", self.session_id, response);

            let response = response.to_bytes();

            let msg = crate::Message {
                request_id: self.next_request_id.read().await.clone(),
                response_id: Some(msg.request_id),
                msg: response,
            };

            let response = msg.to_bytes();

            trace!("{}: Sending: {:?}", self.session_id, msg);

            self.out
                .write()
                .await
                .send(Message::Binary(response))
                .await
                .unwrap();
        }
    }

    pub async fn send<P: ProtoTrait + Send + Serialize + DeserializeOwned>(&self, proto: P) {
        let msg = proto.to_bytes();
        let msg = crate::Message {
            request_id: self.next_request_id.read().await.clone(),
            response_id: None,
            msg,
        };
        let msg = msg.to_bytes();

        *self.next_request_id.write().await += 1;

        debug!("{}: Sending: {:?}", self.session_id, msg);

        self.out
            .write()
            .await
            .send(Message::Binary(msg))
            .await
            .unwrap();
    }

    pub async fn add_query(&self, request_id: u64, handle: JoinHandle<()>) {
        debug!("{}: Adding query: {}", self.session_id, request_id);

        self.queries.write().await.insert(request_id, handle);
    }

    pub async fn abort_queries(&self) {
        for (_, handle) in self.queries.write().await.iter() {
            warn!("{}, aborting query", self.session_id);

            handle.abort();
        }
    }
}
