extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

/// This derives the `FromRow` trait for structs
/// Requires that the query is in field order, as it just uses row indices
#[proc_macro_derive(FromRow)]
pub fn derive_from_row(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    match input.data {
        syn::Data::Struct(data) => match data.fields {
            Fields::Named(ref fields) => {
                let field_vals = fields.named.iter().enumerate().map(|(i, field)| {
                    let name = &field.ident;
                    quote!(#name: row.try_get(#i)?)
                });

                let query_fragment = fields
                    .named
                    .iter()
                    .filter_map(|val| val.ident.as_ref())
                    .map(|val| val.to_string())
                    .collect::<Vec<_>>();

                TokenStream::from(quote!(
                impl divedb_core::FromRow for #name {
                    fn from_row(row: tokio_postgres::Row) -> Result<Self, anyhow::Error> {
                        Ok(Self {
                            #(#field_vals),*
                        })
                    }
                    fn fields() -> &'static [&'static str] {
                        &[
                            #(#query_fragment),*
                        ]
                    }
                }))
            }
            _ => unimplemented!("Only structs can derive `FromRow`"),
        },
        syn::Data::Enum(_) => unimplemented!("Only structs can derive `FromRow`"),
        syn::Data::Union(_) => unimplemented!("Only structs can derive `FromRow`"),
    }
}
