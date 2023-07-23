use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

/// This derives the `FromRow` trait for structs
/// Requires that the query is in field order, as it just uses row indices
#[proc_macro_derive(FromRow)]
pub fn derive_from_row(input: TokenStream) -> TokenStream {
    // Parse it as a proc macro
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref data) = input.data {
        if let Fields::Named(ref fields) = data.fields {
            // Collect up each field and index, and return it.
            let field_vals = fields.named.iter().enumerate().map(|(i, field)| {
                let name = &field.ident;
                quote!(#name: row.try_get(#i)?)
            });

            let name = input.ident;

            return TokenStream::from(quote!(
            impl ::divedb_core::FromRow for #name {
                fn from_row(row: ::tokio_postgres::Row) -> Result<Self, ::anyhow::Error> {
                    Ok(Self {
                        #(#field_vals),*
                    })
                }
            }));
        }
    }

    // We don't care about any other variants, so emit an error here
    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive `FromRow`",
        )
        .to_compile_error(),
    )
}
