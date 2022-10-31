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

                pub fn query<T: From<Model>>(&mut self, query_id: QueryId) -> Option<Vec<T>> {
                    None
                }

                pub fn update<T: Into<Model>>(&mut self, model_id: ModelId, models: Vec<T>) {
                    self.models
                        .entry(model_id)
                        .or_insert_with(BTreeMap::new)
                        .extend(models.into_iter().map(|model| {
                            let model = model.into();

                            (model.id(), model)
                        }));
                }

                pub fn delete(&mut self, model_id: ModelId, ids: Vec<i32>) {
                    self.models
                        .entry(model_id)
                        .or_insert_with(BTreeMap::new)
                        .retain(|id, _| !ids.contains(id));
                }

                pub fn update_for_request_id(&mut self, request_id: RequestId, events: Vec<Event<Model>>) {
                    let query_id = self.requests.get(&request_id).unwrap();

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

                        self.update(model_id, events);
                        self.delete(model_id, deletes);
                    }
                }

                pub fn register_request(&mut self, request_id: RequestId, query_id: QueryId) {
                    self.requests.insert(request_id, query_id);
                }
            }
        }
        pub use cache_mod::Cache;
    })
}
