use std::{collections::HashMap, iter::once};

use darling::{util::PathList, FromAttributes};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Generics, Path};

use crate::transitive::attr::ParsedAttr;

#[derive(FromAttributes)]
#[darling(attributes(transitive))]
pub struct TransitiveTryFrom {
    try_from: PathList,
    with: Option<HashMap<Path, Path>>,
    error: Option<Path>,
}

impl ToTokens for ParsedAttr<'_, &TransitiveTryFrom> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let Generics {
            lt_token, gt_token, ..
        } = self.generics;

        let (generic_parameters, simple_generic_parameters) = match &self.data.with {
            Some(with) => (quote!(), {
                let with = with.iter();
                quote! {#lt_token #(#with),* #gt_token}
            }),
            None => (self.generic_parameters(), self.simple_generic_parameters()),
        };
        let where_clause = &self.generics.where_clause;

        let first = self.data.try_from.first();
        let last = self.data.try_from.last();

        let stmts = self
            .data
            .try_from
            .iter()
            .skip(1)
            .map(|ty| quote! {let val: #ty = core::convert::TryFrom::try_from(val)?;})
            .chain(once(
                quote! {let val = core::convert::TryFrom::try_from(val)?;},
            ));

        let error = self
            .data
            .error
            .as_ref()
            .map(|e| quote!(#e))
            .unwrap_or_else(|| quote!(<Self as TryFrom<#last>>::Error));

        let expanded = quote! {
            impl #generic_parameters core::convert::TryFrom<#first> for #name #simple_generic_parameters
            #where_clause {
                type Error = #error;

                fn try_from(val: #first) -> core::result::Result<Self, Self::Error> {
                    #(#stmts)*
                    Ok(val)
                }
            }
        };

        tokens.extend(expanded);
    }
}
