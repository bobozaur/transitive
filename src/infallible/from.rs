use std::iter::once;

use darling::{util::PathList, FromAttributes};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::transitive::TransitiveIdent;

#[derive(FromAttributes)]
#[darling(attributes(transitive))]
pub struct TransitiveFrom {
    from: PathList,
}

impl ToTokens for TransitiveIdent<'_, &TransitiveFrom> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let first = self.data.from.first();

        let stmts = self
            .data
            .from
            .iter()
            .map(|ty| quote! {let val = #ty::from(val);})
            .chain(once(quote! {let val = #name::from(val);}));

        let expanded = quote! {
            impl core::convert::From<#first> for #name {
                fn from(val: #first) -> #name {
                    #(#stmts)*
                    val
                }
            }
        };

        tokens.extend(expanded);
    }
}
