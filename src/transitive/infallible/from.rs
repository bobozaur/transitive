use std::iter::once;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result as SynResult, Token, Type,
};

use crate::transitive::TokenizablePath;

/// Path corresponding to a [`#[transitive(from(..))`] path.
pub struct TransitionFrom(Punctuated<Type, Token![,]>);

impl Parse for TransitionFrom {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Punctuated::parse_terminated(input).map(Self)
    }
}

impl ToTokens for TokenizablePath<'_, &TransitionFrom> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let first = self.path.0.first();

        let stmts = self
            .path
            .0
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
