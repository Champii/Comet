use proc_macro::TokenStream;

use quote::quote;
use syn::parse::Result;

pub fn perform(_input: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(
        generate_cache().unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}

pub fn generate_cache() -> Result<proc_macro2::TokenStream> {
    Ok(quote! {
        mod cache_mod {
            use std::collections::BTreeMap;
            use std::collections::BTreeSet;
            use std::fmt::Debug;
            use crate::ModelId;
            use crate::Model;
            use comet::prelude::*;
            use crate::RPCQuery;

            //tmp
            pub type QueryId = u64;
            pub type RequestId = u64;

            #[derive(Debug)]
            pub struct Cache {
                models: BTreeMap<ModelId, BTreeMap<i32, Model>>,
                // requests: BTreeMap<RequestId, QueryId>, // request_id -> query_id
                requests: BTreeMap<RequestId, (QueryId, u64)>, // request_id -> (query_id, args_hash)
                queries: BTreeMap<(QueryId, u64), (ModelId, BTreeSet<i32>)>, // (query_id, args_hash) -> (model_id, ids)
            }

            impl Cache {
                pub fn new() -> Self {
                    Self {
                        models: BTreeMap::new(),
                        requests: BTreeMap::new(),
                        queries: BTreeMap::new(),
                    }
                }

                pub fn query<T: From<Model> + Debug>(&self, query_id: QueryId, args_hash: u64) -> Option<Vec<T>> {
                    trace!("Cache query: query_id: {:?} args_hash: {}", query_id, args_hash);
                    let (model_id, ids) = self.queries.get(&(query_id, args_hash))?;

                    debug!("Cache query: model_id: {:?} ids: {:?}", model_id, ids);

                    let res = self.models
                        .get(model_id)?
                        .into_iter()
                        .filter(|(id, _)| ids.contains(id))
                        .map(|(_, model)| T::from(model.clone()))
                        .collect();

                    debug!("Cache query result: {:?}", res);

                    Some(res)
                }

                pub fn update<T: Into<Model> + Debug>(&mut self, query_id: QueryId, args_hash: u64, model_id: ModelId, models: Vec<T>) {
                    trace!("Cache update: query_id: {:?} args_hash {:?} model_id: {:?} models: {:?}", query_id, args_hash, model_id, models);
                    let (model_id_query, ids) = self.queries.get_mut(&(query_id, args_hash)).unwrap();

                    debug!("Cache update query: model_id_query: {:?} ids: {:?}", model_id_query, ids);

                    let ids_models = models.into_iter().map(|model| {
                        let model: Model = model.into();

                        #[allow(unreachable_code)]

                        (model.id(), model)
                    }).collect::<Vec<_>>();

                    debug!("Cache update id_models: {:?}", ids_models);

                    ids.extend(ids_models.iter().map(|(id, _)| *id));

                    if *model_id_query == ModelId::default() {
                        *model_id_query = model_id;
                    }
                    self.models
                        .entry(model_id)
                        .or_insert_with(BTreeMap::new)
                        .extend(ids_models);

                }

                pub fn delete(&mut self, query_id: QueryId, args_hash: u64, model_id: ModelId, ids: Vec<i32>) {
                    trace!("Cache delete: query_id: {:?} args_hash: {:?} model_id: {:?} ids: {:?}", query_id, args_hash, model_id, ids);
                    self.models
                        .entry(model_id)
                        .or_insert_with(BTreeMap::new)
                        .retain(|id, _| !ids.contains(id));

                    // self.queries.get(query_id).unwrap().1.iter().(models.into_iter().map(|model| model.into().id()));
                }

                pub fn update_for_request_id(&mut self, request_id: RequestId, events: Vec<Event<Model>>) {
                    trace!("Cache update_for_request_id: request_id: {:?} events: {:?}", request_id, events);
                    let (query_id, args_hash) = if let Some((query_id, args_hash)) = self.requests.get(&request_id) {
                        (*query_id, *args_hash)
                    } else {
                        return;
                    };

                    let (upsert, deletes): (Vec<_>, Vec<_>) = events.into_iter().partition(|event| !event.is_delete());

                    trace!("Cache update_for_request_id: upsert: {:?} deletes: {:?}", upsert, deletes);

                    let events = upsert
                        .into_iter()
                        .map(|event| match event {
                            Event::Insert(model) => model,
                            Event::Update(model) => model,
                            _ => unreachable!(),
                        }).collect::<Vec<_>>();

                    debug!("Cache update_for_request_id events: {:?}", events);

                    let deletes = deletes
                        .into_iter()
                        .map(|event| match event {
                            Event::Delete(id) => id,
                            _ => unreachable!(),
                        }).collect::<Vec<_>>();

                    debug!("Cache update_for_request_id deletes: {:?}", deletes);

                    if let Some(first) = events.first() {
                        let model_id = first.model_id();

                        debug!("Cache update_for_request_id model_id: {:?}", model_id);

                        self.update(query_id, args_hash, model_id, events);
                        self.delete(query_id, args_hash, model_id, deletes);
                    }
                }

                pub fn register_request(&mut self, request_id: RequestId, query_id: QueryId, args_hash: u64) {
                    trace!("Cache register_request: request_id: {:?} query_id: {:?} args_hash {:?}", request_id, query_id, args_hash);
                    if self.queries.contains_key(&(query_id, args_hash)) {
                        return;
                    }

                    self.requests.insert(request_id, (query_id, args_hash));
                    self.queries.insert((query_id, args_hash), (ModelId::default(), BTreeSet::new()));
                }
            }
        }
        pub use cache_mod::Cache;
    })
}
