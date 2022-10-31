use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, BTreeSet},
    hash::{Hash, Hasher},
};

use crate::prelude::ProtoTrait;

// client check local cache for the QueryHash
// if query exists with given params
//   register the componentId with the QueryHash in the ComponentQueryBinder
//   return the objects from the ModelCache
// if not
//   client create message with query
//   send the request to the server
//   register the componentId in the ComponentQueryBinder
//
// server receive the message
// if watch exists with given params for this user
//   ignore the message
// else
//   create the watch
//   return the result to the client
//   the watch callback send the client the updated results with the QueryHash
//
// client receive the result
//   update the ModelCache
//   get every componentId from the ComponentQueryBinder that are bound to the QueryHash
//   trigger the components render
//

pub type QueryId = u64;
pub type ComponentId = String;
/*
// The message that is sent to the server and back
pub enum Message {
    Query(Query),
    Response(Vec<Model>),
    Event(QueryHash, Vec<Model>, Vec<i32>), // query Hash, new/update models, deleted ids
}
 */
/* #[derive(Hash)]
pub struct Query {
    query_id: QueryId,
    params: Vec<String>,
    model_id: ModelId,
}

impl Query {
    pub fn calc_hash(&self) -> QueryHash {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
} */

// pub type QueryHash = u64;

pub struct Cache {
    query: QueryCache,
    component_query_binder: ComponentQueryBinder,
    pub models: ModelCache,
}

/* pub type ModelId = u64;

// generate_model_enum!();
#[derive(Debug, Clone)]
pub enum Model {
    // Model::User(User),
}

impl Model {
    pub fn id(&self) -> i32 {
        match self {
            // Model::User(user) => user.id,
            _ => 0,
        }
    }
}
 */
// client check local cache for the queryId
impl Cache {
    // returns the component_ids that needs to be redrawn
    pub fn handle_message(&mut self, proto: Proto) -> Option<BTreeSet<ComponentId>> {
        match proto {
            Proto::Event(request_id, event) => {
                self.models.update(query_hash, models.clone(), deleted_ids);
                self.query
                    .insert_ids(query_hash, models.iter().map(|model| model.id()).collect());

                self.component_query_binder
                    .get_components(query_hash)
                    .cloned()
            }
            _ => unimplemented!(),
        }
    }

    pub async fn query(
        &mut self,
        // query: Query,
        component_id: ComponentId,
    ) -> Result<Vec<Model>, ()> {
        let query_hash = query.calc_hash();

        if self.query.contains(&query_hash) {
            self.component_query_binder.bind(query_hash, component_id);

            return self.models.get(query.model_id).ok_or(());
        } else {
            self.query.insert_query(query);

            self.component_query_binder.bind(query_hash, component_id);

            // send the query to the server
            /* if let Ok(Message::Response(models)) = crate::SOCKET.rpc(Message::Query(query)).await {
                self.models.update(query.model_id, models.clone(), vec![]);

                return Ok(models);
            } */
        }

        Err(())
    }
}

pub struct ComponentQueryBinder {
    bound: BTreeMap<QueryHash, BTreeSet<ComponentId>>,
}

impl ComponentQueryBinder {
    fn bind(&mut self, query_hash: QueryHash, component_id: ComponentId) {
        self.bound
            .entry(query_hash)
            .or_insert(BTreeSet::new())
            .insert(component_id);
    }

    fn get_components(&self, query_hash: QueryHash) -> Option<&BTreeSet<ComponentId>> {
        self.bound.get(&query_hash)
    }
}

pub struct ModelCache {
    models: BTreeMap<ModelId, BTreeMap<i32, Model>>,
}

impl ModelCache {
    pub fn update(&mut self, model_id: ModelId, models: Vec<Model>, deleted_ids: Vec<i32>) {
        self.models
            .entry(model_id)
            .or_insert(BTreeMap::new())
            .extend(
                models
                    .into_iter()
                    .map(|model| (model.id(), model))
                    .collect::<Vec<_>>(),
            );

        deleted_ids.into_iter().for_each(|id| {
            self.models
                .entry(model_id)
                .or_insert(BTreeMap::new())
                .remove(&id);
        });
    }

    pub fn get(&self, model_id: ModelId) -> Option<Vec<Model>> {
        self.models.get(&model_id).map(|models| {
            models
                .iter()
                .map(|(_, model)| model.clone())
                .collect::<Vec<Model>>()
        })
    }
}

pub struct QueryCache {
    queries: BTreeMap<QueryHash, Query>,
    cache: BTreeMap<QueryHash, BTreeSet<i32>>, // store the ids of the models
}

impl QueryCache {
    pub fn insert_query(&mut self, query: Query) {
        self.queries.insert(query.calc_hash(), query);
    }

    pub fn insert_ids(&mut self, query_hash: QueryHash, ids: BTreeSet<i32>) {
        self.cache
            .entry(query_hash)
            .or_insert(BTreeSet::new())
            .extend(ids.clone());
    }

    pub fn contains(&self, query_hash: &QueryHash) -> bool {
        self.cache.contains_key(query_hash)
    }

    // get the watched models' ids for the given query
    pub fn get_query_ids(&self, query_hash: &QueryHash) -> Option<&BTreeSet<i32>> {
        self.cache.get(query_hash)
    }
}
