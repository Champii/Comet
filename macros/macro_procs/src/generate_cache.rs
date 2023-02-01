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
            use crate::ModelId;
            use crate::Model;
            use comet::prelude::Event;

            //tmp
            pub type QueryId = u64;
            pub type RequestId = u64;

            #[derive(Debug)]
            pub struct Cache {
                models: BTreeMap<ModelId, BTreeMap<i32, Model>>,
                requests: BTreeMap<RequestId, QueryId>, // request_id -> query_id
                queries: BTreeMap<QueryId, (ModelId, BTreeSet<i32>)>, // query_id -> (model_id, ids)
            }

            impl Cache {
                pub fn new() -> Self {
                    Self {
                        models: BTreeMap::new(),
                        requests: BTreeMap::new(),
                        queries: BTreeMap::new(),
                    }
                }

                pub fn query<T: From<Model>>(&self, query_id: QueryId) -> Option<Vec<T>> {
                    let (model_id, ids) = self.queries.get(&query_id)?;

                    Some(self.models
                        .get(model_id)?
                        .into_iter()
                        .filter(|(id, _)| ids.contains(id))
                        .map(|(_, model)| T::from(model.clone()))
                        .collect())
                }

                pub fn update<T: Into<Model>>(&mut self, query_id: QueryId, model_id: ModelId, models: Vec<T>) {
                    let (model_id_query, ids) = self.queries.get_mut(&query_id).unwrap();

                    let ids_models = models.into_iter().map(|model| {
                        let model: Model = model.into();

                        #[allow(unreachable_code)]

                        (model.id(), model)
                    }).collect::<Vec<_>>();

                    ids.extend(ids_models.iter().map(|(id, _)| *id));

                    if *model_id_query == ModelId::default() {
                        *model_id_query = model_id;
                    }
                    self.models
                        .entry(model_id)
                        .or_insert_with(BTreeMap::new)
                        .extend(ids_models);

                }

                pub fn delete(&mut self, query_id: QueryId, model_id: ModelId, ids: Vec<i32>) {
                    self.models
                        .entry(model_id)
                        .or_insert_with(BTreeMap::new)
                        .retain(|id, _| !ids.contains(id));

                    // self.queries.get(query_id).unwrap().1.iter().(models.into_iter().map(|model| model.into().id()));
                }

                pub fn update_for_request_id(&mut self, request_id: RequestId, events: Vec<Event<Model>>) {
                    let query_id = if let Some(query_id) = self.requests.get(&request_id) {
                        *query_id
                    } else {
                        return;
                    };

                    let (upsert, deletes): (Vec<_>, Vec<_>) = events.into_iter().partition(|event| !event.is_delete());

                    let events = upsert
                        .into_iter()
                        .map(|event| match event {
                            Event::Insert(model) => model,
                            Event::Update(model) => model,
                            _ => unreachable!(),
                        }).collect::<Vec<_>>();

                    let deletes = deletes
                        .into_iter()
                        .map(|event| match event {
                            Event::Delete(id) => id,
                            _ => unreachable!(),
                        }).collect::<Vec<_>>();

                    if let Some(first) = events.first() {
                        let model_id = first.model_id();

                        self.update(query_id, model_id, events);
                        self.delete(query_id, model_id, deletes);
                    }
                }

                pub fn register_request(&mut self, request_id: RequestId, query_id: QueryId) {
                    if self.queries.contains_key(&query_id) {
                        return;
                    }

                    self.requests.insert(request_id, query_id);
                    self.queries.insert(query_id, (ModelId::default(), BTreeSet::new()));
                }
            }
        }
        pub use cache_mod::Cache;
    })
}
