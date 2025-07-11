use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

use super::FallibleTypeList;
use crate::transitive::{distinct_types_check, TokenizablePath};

/// Path corresponding to a [`#[transitive(try_into(..))`] path.
pub struct TryTransitionInto(FallibleTypeList);

impl Parse for TryTransitionInto {
    fn parse(input: ParseStream) -> SynResult<Self> {
        FallibleTypeList::parse(input).map(Self)
    }
}

impl ToTokens for TokenizablePath<'_, &TryTransitionInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let first = &self.path.0.first_type;
        let last = &self.path.0.last_type;
        let second_last = self.path.0.intermediate_types.last().unwrap_or(first);

        let stmts = std::iter::once(first)
            .chain(&self.path.0.intermediate_types)
            .map(|ty| quote! {let val: #ty = core::convert::TryFrom::try_from(val)?;});

        let error = self
            .path
            .0
            .error
            .as_ref()
            .map(|e| quote!(#e))
            .unwrap_or_else(|| quote!(<#last as TryFrom<#second_last>>::Error));

        let types_check = distinct_types_check(
            first,
            last,
            name,
            &impl_generics,
            &ty_generics,
            where_clause,
        );

        let expanded = quote! {
            impl #impl_generics core::convert::TryFrom<#name #ty_generics> for #last #where_clause {
                type Error = #error;

                fn try_from(val: #name #ty_generics) -> core::result::Result<Self, Self::Error> {
                    #types_check
                    #(#stmts)*
                    let val = core::convert::TryFrom::try_from(val)?;
                    Ok(val)
                }
            }
        };

        tokens.extend(expanded);
    }
}
