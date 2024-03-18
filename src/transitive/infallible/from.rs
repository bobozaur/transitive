use std::iter::once;

use darling::{util::PathList, FromAttributes};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::transitive::attr::ParsedAttr;

#[derive(FromAttributes)]
#[darling(attributes(transitive_from))]
pub struct TransitiveFrom {
    path: PathList,
}

impl ToTokens for ParsedAttr<'_, &TransitiveFrom> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let first = self.data.path.first();

        let stmts = self
            .data
            .path
            .iter()
            .map(|ty| quote! {let val: #ty = core::convert::From::from(val);})
            .chain(once(quote! {core::convert::From::from(val)}));

        let expanded = quote! {
            impl #impl_generics core::convert::From<#first> for #name #ty_generics #where_clause {
                fn from(val: #first) -> Self {
                    #(#stmts)*
                }
            }
        };

        tokens.extend(expanded);
    }
}
