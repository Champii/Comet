use {
    futures::stream::StreamExt, pharos::*, wasm_bindgen::UnwrapThrowExt,
    wasm_bindgen_futures::spawn_local, ws_stream_wasm::*,
};

use futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    SinkExt,
};
use serde::{de::DeserializeOwned, Serialize};

use std::{collections::HashMap, fmt::Debug};

use crate::prelude::*;

use crate::ProtoTrait;

use crate::Message;

pub struct Socket<P: ProtoTrait + 'static + Serialize + DeserializeOwned> {
    tx: UnboundedSender<Message>,
    rx: Option<UnboundedReceiver<Message>>,
    next_request_id: u64,
    pending_requests: Arc<RwLock<HashMap<u64, futures::channel::oneshot::Sender<Message>>>>,
    _phantom: std::marker::PhantomData<P>,
}

impl<P: ProtoTrait + 'static + Serialize + DeserializeOwned> Socket<P>
where
    Self: 'static,
{
    pub fn new(
        tx: UnboundedSender<Message>,
        rx: Option<UnboundedReceiver<Message>>,
        pending_requests: Arc<RwLock<HashMap<u64, futures::channel::oneshot::Sender<Message>>>>,
    ) -> Self {
        Self {
            tx,
            rx,
            next_request_id: 0,
            pending_requests,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn connect(url: String) -> Self {
        // in to the socket or out of the socket
        let (in_tx, mut in_rx) = futures::channel::mpsc::unbounded::<Message>();
        let (mut out_tx, out_rx) = futures::channel::mpsc::unbounded();

        let (mut ws, wsio) = WsMeta::connect(url, None)
            .await
            .expect_throw("assume the connection succeeds");

        let _evts = ws.observe(ObserveConfig::default()).await.unwrap();

        let (mut ws_tx, mut ws_rx) = wsio.split();

        let input_loop = async move {
            while let Some(msg) = in_rx.next().await {
                ws_tx.send(WsMessage::Binary(msg.to_bytes())).await.unwrap();
            }
        };

        spawn_local(input_loop);

        let pending_requests = Arc::new(RwLock::new(HashMap::new()));
        let pending_requests2 = Arc::clone(&pending_requests);

        let output_loop = async move {
            while let Some(msg) = ws_rx.next().await {
                if let WsMessage::Binary(blob) = msg {
                    let msg = Message::from_bytes(&blob);

                    if let Some(response_id) = msg.response_id {
                        if pending_requests2.read().await.contains_key(&response_id) {
                            let tx: futures::channel::oneshot::Sender<Message> = pending_requests2
                                .write()
                                .await
                                .remove(&response_id)
                                .unwrap();

                            tx.send(msg).unwrap();
                        } else {
                            out_tx.send(msg).await.unwrap();
                        }
                    } else {
                        out_tx.send(msg).await.unwrap();
                    }
                } else {
                    // bad message type
                }
            }
        };

        spawn_local(output_loop);

        Self::new(in_tx, Some(out_rx), pending_requests)
    }

    pub async fn rpc(&mut self, packet: P) -> P {
        let request_id = self.next_request_id;
        self.next_request_id += 1;

        let (tx, rx) = futures::channel::oneshot::channel::<Message>();
        let (future, _handle) =
            futures::future::abortable(async move { P::from_bytes(&rx.await.unwrap().msg) });

        // if timeout then abort the handle
        /* spawn_local(async move {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            handle.abort();
        }); */

        self.pending_requests.write().await.insert(request_id, tx);

        let msg = Message {
            request_id,
            response_id: None,
            msg: packet.to_bytes(),
        };

        self.tx.send(msg).await.unwrap();

        future.await.unwrap()
    }

    pub async fn send(&mut self, packet: P) -> P {
        self.rpc(packet).await
    }

    pub fn take_receiver(&mut self) -> Option<UnboundedReceiver<Message>> {
        self.rx.take()
    }

    pub fn get_next_request_id(&self) -> u64 {
        self.next_request_id
    }
}
