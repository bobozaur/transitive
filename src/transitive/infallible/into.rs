use std::collections::HashMap;

use darling::{util::PathList, FromAttributes};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Generics, Path};

use crate::transitive::attr::ParsedAttr;

#[derive(FromAttributes)]
#[darling(attributes(transitive))]
pub struct TransitiveInto {
    into: PathList,
    with: Option<HashMap<Path, Path>>,
}

impl ToTokens for ParsedAttr<'_, &TransitiveInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let Generics {
            lt_token, gt_token, ..
        } = self.generics;

        let (generic_parameters, simple_generic_parameters) = match &self.data.with {
            Some(with) => (quote!(), quote! {#lt_token #(#with),* #gt_token}),
            None => (self.generic_parameters(), self.simple_generic_parameters()),
        };
        let where_clause = &self.generics.where_clause;

        let last = self.data.into.last();

        let stmts = self
            .data
            .into
            .iter()
            .take(self.data.into.len() - 1)
            .map(|ty| quote! {let val: #ty = core::convert::From::from(val);});

        let expanded = quote! {
            impl #generic_parameters core::convert::From<#name #simple_generic_parameters> for #last
            #where_clause {
                fn from(val: #name #simple_generic_parameters) -> #last {
                    #(#stmts)*
                    core::convert::From::from(val)
                }
            }
        };

        tokens.extend(expanded);
    }
}
