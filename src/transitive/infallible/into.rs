use darling::{util::PathList, FromAttributes};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::transitive::attr::ParsedAttr;

#[derive(FromAttributes)]
#[darling(attributes(transitive_into))]
pub struct TransitiveInto {
    path: PathList,
}

impl ToTokens for ParsedAttr<'_, &TransitiveInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let last = self.data.path.last();

        let stmts = self
            .data
            .path
            .iter()
            .take(self.data.path.len() - 1)
            .map(|ty| quote! {let val: #ty = core::convert::From::from(val);});

        let expanded = quote! {
            impl #impl_generics core::convert::From<#name #ty_generics> for #last #where_clause {
                fn from(val: #name #ty_generics) -> #last {
                    #(#stmts)*
                    core::convert::From::from(val)
                }
            }
        };

        tokens.extend(expanded);
    }
}
