use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

use super::TypeList;
use crate::transitive::{distinct_types_check, TokenizablePath};

/// Path corresponding to a [`#[transitive(into(..))`] path.
pub struct TransitionInto(TypeList);

impl Parse for TransitionInto {
    fn parse(input: ParseStream) -> SynResult<Self> {
        TypeList::parse(input).map(Self)
    }
}

impl ToTokens for TokenizablePath<'_, &TransitionInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let first = &self.path.0.first_type;
        let last = &self.path.0.last_type;

        let stmts = std::iter::once(first)
            .chain(&self.path.0.intermediate_types)
            .map(|ty| quote! {let val: #ty = core::convert::From::from(val);});

        let types_check = distinct_types_check(
            first,
            last,
            name,
            &impl_generics,
            &ty_generics,
            where_clause,
        );

        let expanded = quote! {
            impl #impl_generics core::convert::From<#name #ty_generics> for #last #where_clause {
                fn from(val: #name #ty_generics) -> #last {
                    #types_check
                    #(#stmts)*
                    core::convert::From::from(val)
                }
            }
        };

        tokens.extend(expanded);
    }
}
