use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Path, Result as SynResult, Token,
};

use crate::transitive::TokenizablePath;

/// Path corresponding to a [`#[transitive(into(..))`] path.
pub struct TransitionInto(Punctuated<Path, Token![,]>);

impl Parse for TransitionInto {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Punctuated::parse_terminated(input).map(Self)
    }
}

impl ToTokens for TokenizablePath<'_, &TransitionInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let last = self.path.0.last();

        let stmts = self
            .path
            .0
            .iter()
            .take(self.path.0.len() - 1)
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
