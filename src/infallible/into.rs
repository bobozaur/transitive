use darling::{util::PathList, FromAttributes};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::transitive::TransitiveIdent;

#[derive(FromAttributes)]
#[darling(attributes(transitive))]
pub struct TransitiveInto {
    into: PathList,
}

impl ToTokens for TransitiveIdent<'_, &TransitiveInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;

        let last = self.data.into.last();

        let stmts = self
            .data
            .into
            .iter()
            .map(|ty| quote! {let val = #ty::from(val);});

        let expanded = quote! {
            impl core::convert::From<#name> for #last {
                fn from(val: #name) -> #last {
                    #(#stmts)*
                    val
                }
            }
        };

        tokens.extend(expanded);
    }
}
