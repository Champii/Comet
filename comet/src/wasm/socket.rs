use {
    futures::stream::StreamExt, pharos::*, wasm_bindgen::UnwrapThrowExt,
    wasm_bindgen_futures::spawn_local, ws_stream_wasm::*,
};

use futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    SinkExt,
};

use crate::Proto;

pub struct Socket<P: Proto + 'static> {
    tx: UnboundedSender<P>,
    rx: Option<UnboundedReceiver<P>>,
}

impl<P: Proto + 'static> Socket<P> {
    pub fn new(tx: UnboundedSender<P>, rx: Option<UnboundedReceiver<P>>) -> Self {
        Self { tx, rx }
    }

    pub async fn connect(url: String) -> Self {
        // in to the socket or out of the socket
        let (in_tx, mut in_rx) = futures::channel::mpsc::unbounded::<P>();
        let (mut out_tx, out_rx) = futures::channel::mpsc::unbounded();

        let (mut ws, wsio) = WsMeta::connect(url, None)
            .await
            .expect_throw("assume the connection succeeds");

        let _evts = ws.observe(ObserveConfig::default()).await.unwrap();

        let (mut ws_tx, mut ws_rx) = wsio.split();

        let input_loop = async move {
            while let Some(packet) = in_rx.next().await {
                ws_tx.send(WsMessage::Binary(packet.to_bytes())).await.unwrap();
            }
        };

        spawn_local(input_loop);

        let output_loop = async move {
            while let Some(msg) = ws_rx.next().await {
                if let WsMessage::Binary(blob) = msg {
                    let packet = P::from_bytes(&blob);

                    out_tx.send(packet).await.unwrap();
                } else {
                    // bad message type
                }
            }
        };

        spawn_local(output_loop);

        Self::new(in_tx, Some(out_rx))
    }

    pub async fn send(&mut self, packet: P) {
        self.tx.send(packet).await.unwrap();
    }

    pub fn send_async(&mut self, packet: P) {
        let mut tx = self.tx.clone();

        spawn_local(async move {
            tx.send(packet).await.unwrap();
        });
    }

    pub fn take_receiver(&mut self) -> Option<UnboundedReceiver<P>> {
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
