use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use super::client::Client;

lazy_static! {
    pub static ref UNIVERSE: Universe = Universe::default();
}

pub type Universe = Arc<RwLock<UniverseInner>>;

#[derive(Clone, Debug, Default)]
pub struct UniverseInner {
    next_session_id: usize,
    clients: HashMap<usize, Client>, // session_id -> Client
}

impl UniverseInner {
    pub fn get_next_session_id(&mut self) -> usize {
        let id = self.next_session_id;

        self.next_session_id += 1;

        id
    }

    pub fn new_client(&mut self, mut client: Client) -> usize {
        let session_id = self.get_next_session_id();

        client.set_session_id(session_id);

        self.clients.insert(session_id, client);

        session_id
    }

    pub fn get_client(&self, session_id: usize) -> Client {
        self.clients.get(&session_id).unwrap().clone()
    }

    pub async fn remove_client(&mut self, session_id: usize) {
        let client = self.clients.remove(&session_id).unwrap();

        client.abort_queries().await;
    }
}
