use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

use super::TypeList;
use crate::transitive::TokenizablePath;

/// Path corresponding to a [`#[transitive(from(..))`] path.
pub struct TransitionFrom(TypeList);

impl Parse for TransitionFrom {
    fn parse(input: ParseStream) -> SynResult<Self> {
        TypeList::parse(input).map(Self)
    }
}

impl ToTokens for TokenizablePath<'_, &TransitionFrom> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let first = &self.path.0.first_type;
        let last = &self.path.0.last_type;

        let stmts = self
            .path
            .0
            .intermediate_types
            .iter()
            .chain(std::iter::once(last))
            .map(|ty| quote! {let val: #ty = core::convert::From::from(val);})
            .chain(std::iter::once(quote! {core::convert::From::from(val)}));

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
