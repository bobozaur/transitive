use darling::{util::PathList, FromAttributes};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::transitive::attr::ParsedAttr;

#[derive(FromAttributes)]
#[darling(attributes(transitive))]
pub struct TransitiveInto {
    into: PathList,
}

impl ToTokens for ParsedAttr<'_, &TransitiveInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let generic_parameters = self.generic_parameters();

        let last = self.data.into.last();

        let stmts = self
            .data
            .into
            .iter()
            .take(self.data.into.len() - 1)
            .map(|ty| quote! {let val: #ty = core::convert::From::from(val);});

        let expanded = quote! {
            impl #generic_parameters core::convert::From<#name #generic_parameters> for #last {
                fn from(val: #name #generic_parameters) -> #last {
                    #(#stmts)*
                    core::convert::From::from(val)
                }
            }
        };

        tokens.extend(expanded);
    }
}
