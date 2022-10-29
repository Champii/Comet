use proc_macro::TokenStream;

use quote::quote;
use syn::{parse::Result, parse_macro_input, Fields, ItemStruct};

pub fn perform(table_name: String, input: TokenStream) -> TokenStream {
    let mcall = parse_macro_input!(input as syn::ItemStruct);

    proc_macro::TokenStream::from(
        impl_model_macro(table_name, mcall.clone())
            .unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}

fn impl_model_macro(
    table_name: String,
    mut item_struct: ItemStruct,
) -> Result<proc_macro2::TokenStream> {
    let name = item_struct.ident.clone();
    // let name2 = ast.ident.clone();

    crate::generate_migrations::register_migration(item_struct.clone());

    let fields = &item_struct.fields;
    let derives = item_struct.attrs.clone();
    let derives2 = derives.clone();

    let res = match fields {
        Fields::Named(fields) => {
            let named = &fields.named;

            let lower_name = name.to_string().to_ascii_lowercase();

            let lower_name_ident: syn::Ident = syn::parse_str(&lower_name).unwrap();

            // tricks to have a one-line iterator to create "impl #new_name_ident {}"
            let mut lower_name_ident_vec = vec![];

            for _ in &fields.named {
                lower_name_ident_vec.push(lower_name_ident.clone());
            }

            // tricks to have a one-line iterator to create "pub fn fetch_by_ {}"
            let mut name_vec = vec![];

            for _ in &fields.named {
                name_vec.push(name.clone());
            }

            let new_name = "New".to_string() + &name.to_string();
            item_struct.ident = syn::parse_str(&new_name).unwrap();

            let new_name_ident = item_struct.ident.clone();

            let table_name = if table_name.len() == 0 {
                lower_name.clone() + &"s"
            } else {
                table_name.replace("\"", "")
            };

            let table_name_ident: syn::Ident = syn::parse_str(&table_name).unwrap();

            // tricks to have a one-line iterator to create "pub fn fetch_by_ {}"
            let mut table_name_ident_vec = vec![];

            for _ in &fields.named {
                table_name_ident_vec.push(table_name_ident.clone());
            }

            let from_name: syn::Ident =
                syn::parse_str(&("from_".to_string() + &lower_name)).unwrap();

            let mut fields_idents = vec![];

            for field in &fields.named {
                fields_idents.push(field.ident.clone().unwrap());
            }

            let fields_idents2 = fields_idents.clone();

            let res = quote!(
                #[cfg(target_arch = "wasm32")]
                mod #lower_name_ident {
                    use super::*;

                    #(#derives)*
                    #[derive(Serialize, Deserialize, Clone)]
                    #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
                    pub struct #name {
                        pub id: i32,
                        #named
                    }

                    // #(#derives2)*
                    #[derive(Clone, Serialize, Deserialize)]
                    #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
                    #item_struct
                }

                #[cfg(target_arch = "wasm32")]
                pub use #lower_name_ident::{#name, #new_name_ident};

                #[cfg(not(target_arch = "wasm32"))]
                mod #lower_name_ident {
                    use super::*;

                    use comet::prelude::diesel::prelude::*;
                    use crate::schema::#table_name_ident;

                    #(#derives2)*
                    #[derive(Identifiable, Serialize, Deserialize, Queryable, Clone, AsChangeset)]
                    #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
                    // #[diesel(table_name = #table_name)]
                    #[diesel(treat_none_as_null = true)]
                    pub struct #name {
                        pub id: i32,
                        #named
                    }

                    #[derive(Insertable, Clone, Serialize, Deserialize, AsChangeset)]
                    #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
                    #[diesel(table_name = #table_name_ident)]
                    #[diesel(treat_none_as_null = true)]
                    #item_struct

                    impl #new_name_ident {
                        pub fn #from_name(#lower_name_ident: &#name) -> #new_name_ident {
                            #new_name_ident {
                                #(#fields_idents: #lower_name_ident_vec.#fields_idents2.clone()),*
                            }
                        }
                    }
                }

                // use crate::{RPCQuery, RPCResult, Proto};

                #[rpc]
                impl #name {
                    pub async fn create(&self) -> std::result::Result<#name, String> {
                        use crate::schema::#table_name_ident;

                        let mut conn = crate::establish_connection();

                        let #lower_name_ident = #new_name_ident::#from_name(self);

                        diesel::insert_into(#table_name_ident::table)
                            .values(#lower_name_ident)
                            .execute(&mut conn).map_err(|e| "Error insert".to_string())?;

                        #table_name_ident::table
                            .order(#table_name_ident::dsl::id.desc())
                            .first(&mut conn).map_err(|e| "Error create".to_string())
                    }

                    pub async fn list() -> std::result::Result<Vec<#name>, String> {
                        use crate::schema::#table_name_ident;

                        let mut conn = crate::establish_connection();


                        #table_name_ident::table
                            .order(#table_name_ident::dsl::id)
                            .load::<#name>(&mut conn).map_err(|e| "Error list".to_string())
                    }

                    pub async fn update(id_given: i32, #lower_name_ident: #name) -> std::result::Result<usize, String> {
                        use crate::schema::#table_name_ident;

                        let mut conn = crate::establish_connection();

                        let #lower_name_ident = #new_name_ident::#from_name(&#lower_name_ident);

                        diesel::update(#table_name_ident::table.find(id_given))
                            .set(&#lower_name_ident)
                            .execute(&mut conn).map_err(|e| "Error update".to_string())
                    }

                    pub async fn save(&mut self) -> std::result::Result<(), String> {
                        use crate::schema::#table_name_ident;

                        if self.id == -1 {
                            let res = self.create().await?;

                            self.id = res.id;

                        } else {
                            #name::update(self.id, self.clone()).await?;
                        }

                        Ok(())
                    }

                    pub async fn delete(id_given: i32) -> std::result::Result<usize, String> {
                        use crate::schema::#table_name_ident;

                        let mut conn = crate::establish_connection();

                        diesel::delete(#table_name_ident::table.find(id_given))
                            .execute(&mut conn).map_err(|e| "Error delete".to_string())
                    }

                    pub async fn fetch(id_given: i32) -> std::result::Result<#name, String> {
                        use crate::schema::#table_name_ident;

                        let mut conn = crate::establish_connection();

                        #table_name_ident::table.filter(#table_name_ident::dsl::id.eq(id_given)).first::<#name>(&mut conn).map_err(|e| "Error fetch".to_string())
                    }
                }

                #[cfg(not(target_arch = "wasm32"))]
                pub use #lower_name_ident::{#name, #new_name_ident};
            );

            res
        }
        _ => panic!("Orm 'model': The target must be a named struct."),
    };

    Ok(res)
}
