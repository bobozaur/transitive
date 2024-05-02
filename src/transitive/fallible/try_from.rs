use std::iter::once;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

use super::FalliblePathList;
use crate::transitive::TokenizablePath;

/// Path corresponding to a [`#[transitive(try_from(..))`] path.
pub struct TryTransitionFrom(FalliblePathList);

impl Parse for TryTransitionFrom {
    fn parse(input: ParseStream) -> SynResult<Self> {
        FalliblePathList::parse(input).map(Self)
    }
}

impl ToTokens for TokenizablePath<'_, &TryTransitionFrom> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let first = self.path.0.path_list.first();
        let last = self.path.0.path_list.last();

        let stmts = self
            .path
            .0
            .path_list
            .iter()
            .skip(1)
            .map(|ty| quote! {let val: #ty = core::convert::TryFrom::try_from(val)?;})
            .chain(once(
                quote! {let val = core::convert::TryFrom::try_from(val)?;},
            ));

        let error = self
            .path
            .0
            .error
            .as_ref()
            .map(|e| quote!(#e))
            .unwrap_or_else(|| quote!(<Self as TryFrom<#last>>::Error));

        let expanded = quote! {
            impl #impl_generics core::convert::TryFrom<#first> for #name #ty_generics #where_clause {
                type Error = #error;

                fn try_from(val: #first) -> core::result::Result<Self, Self::Error> {
                    #(#stmts)*
                    Ok(val)
                }
            }
        };

        tokens.extend(expanded);
    }
}
