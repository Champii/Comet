use {
    futures::stream::StreamExt, pharos::*, wasm_bindgen::UnwrapThrowExt,
    wasm_bindgen_futures::spawn_local, ws_stream_wasm::*,
};

use futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    Future, SinkExt,
};
use serde::{de::DeserializeOwned, Serialize};

use std::{collections::HashMap, fmt::Debug};

use crate::prelude::*;

use crate::Proto;

use crate::Message;

pub struct Socket<P: Proto + 'static + Serialize + DeserializeOwned + Debug> {
    tx: UnboundedSender<Message>,
    rx: Option<UnboundedReceiver<Message>>,
    next_request_id: u64,
    pending_requests: Arc<RwLock<HashMap<u64, futures::channel::oneshot::Sender<Message>>>>,
    _phantom: std::marker::PhantomData<P>,
}

impl<P: Proto + 'static + Serialize + DeserializeOwned + Debug> Socket<P> {
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

                    /* if pending_requests2
                        .read()
                        .unwrap()
                        .contains_key(&msg.request_id)
                    {
                        let tx: futures::channel::oneshot::Sender<Message> = pending_requests2
                            .write()
                            .unwrap()
                            .remove(&msg.request_id)
                            .unwrap();

                        tx.send(msg).unwrap();
                    } else {
                        out_tx.send(msg).await.unwrap();
                    } */
                    out_tx.send(msg).await.unwrap();
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

        let (tx, mut rx) = futures::channel::oneshot::channel::<Message>();
        let (future, handle) = futures::future::abortable(async move {
            crate::console_log!("RESOLVE RPC");
            /* if let Ok(message) = rx.await {
                let packet = P::from_bytes(&message.msg);
                packet
            } else {
                panic!("no message with id {}", request_id);
            } */
            // panic!("no message with id {}", request_id);
            P::from_bytes(&rx.await.unwrap().msg)
        });

        // if timeoug then abort the handle
        /* let timeout = async move {
            // tokio::time::delay_for(std::time::Duration::from_secs(10)).await;
            handle.abort();
        }; */

        self.pending_requests
            .write()
            .unwrap()
            .insert(request_id, tx);

        let msg = Message {
            request_id,
            msg: packet.to_bytes(),
        };

        crate::console_log!("SEND RPC");
        self.tx.send(msg).await.unwrap();

        crate::console_log!("SEND TAGRANDTANTE");
        let packet = future.await.unwrap();
        crate::console_log!("SEND TAGRANDTANTE {:#?}", packet);

        // P::from_bytes(&message.msg)
        packet
    }
    pub async fn tamere_wesh(&mut self) {
        crate::console_log!("SEND TAMEREJk");
        // let message = self.rx.as_mut().unwrap().next().await.unwrap();
        // crate::console_log!("SEND TAGRANDTANTE {:#?}", message);
    }

    pub async fn send(&mut self, packet: P) -> P {
        // self.tx.send(packet).await.unwrap();
        // unimplemented!()
        self.rpc(packet).await
    }

    /* pub fn send_async(&mut self, packet: P) {
        let mut tx = self.tx.clone();

        spawn_local(async move {
            tx.send(packet).await.unwrap();
        });
    } */

    // pub fn send_sync(&mut self, packet: P) -> P {
    /* let mut tx = self.tx.clone();
    let (tx2, mut rx2) = futures::channel::oneshot::channel::<P>();

    let request_id = self.next_request_id;
    self.next_request_id += 1;

    let (tx3, rx3) = futures::channel::oneshot::channel::<Message>();
    let (future, handle) = futures::future::abortable(async move {
        crate::console_log!("RESOLVE RPC");
        if let Ok(message) = rx3.await {
            let packet = P::from_bytes(&message.msg);
            packet
        } else {
            panic!("no message with id {}", request_id);
        }
    });

    self.pending_requests
        .write()
        .unwrap()
        .insert(request_id, tx3);

    spawn_local(async move {
        let msg = Message {
            request_id,
            msg: packet.to_bytes(),
        };
        tx.send(msg.clone()).await.unwrap();
        tx2.send(P::from_bytes(&msg.msg)).unwrap();
    });

    loop {
        if let Some(packet) = rx2.try_recv().unwrap() {
            return packet;
        } else {
            crate::console_log!("WAITING");
        }
    } */
    // }

    pub fn take_receiver(&mut self) -> Option<UnboundedReceiver<Message>> {
        self.rx.take()
    }

    /* pub async fn next(&mut self) -> Option<Packet> {
        if let Some(rx) = &mut self.rx {
            rx.next().await
        } else {
            None
        }
    } */
}
