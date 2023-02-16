use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub request_id: u64,
    pub response_id: Option<u64>,
    pub msg: Vec<u8>,
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_cbor::to_vec(self).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        serde_cbor::from_slice(bytes).unwrap()
    }
}

#[async_trait]
pub trait ProtoTrait {
    type Response: ProtoTrait + Send + Serialize + DeserializeOwned + Debug;
    type Client;

    async fn dispatch(self, _request_id: u64, _client: Self::Client) -> Option<Self::Response>
    where
        Self: Sized,
        Self::Client: Send,
    {
        None
    }

    fn from_bytes(bytes: &[u8]) -> Self
    where
        Self: Sized + DeserializeOwned,
    {
        serde_cbor::from_slice(bytes).unwrap()
    }

    fn to_bytes(&self) -> Vec<u8>
    where
        Self: Sized + Serialize,
    {
        serde_cbor::to_vec(self).unwrap()
    }
}
