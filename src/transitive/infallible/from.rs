use std::iter::once;

use darling::{util::PathList, FromAttributes};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::transitive::attr::ParsedAttr;

#[derive(FromAttributes)]
#[darling(attributes(transitive))]
pub struct TransitiveFrom {
    from: PathList,
}

impl ToTokens for ParsedAttr<'_, &TransitiveFrom> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let generic_parameters = self.generic_parameters();
        let simple_generic_parameters = self.simple_generic_parameters();
        let where_clause = &self.generics.where_clause;

        let first = self.data.from.first();

        let stmts = self
            .data
            .from
            .iter()
            .map(|ty| quote! {let val: #ty = core::convert::From::from(val);})
            .chain(once(quote! {core::convert::From::from(val)}));

        let expanded = quote! {
            impl #generic_parameters core::convert::From<#first> for #name #simple_generic_parameters 
            #where_clause {
                fn from(val: #first) -> Self {
                    #(#stmts)*
                }
            }
        };

        tokens.extend(expanded);
    }
}
