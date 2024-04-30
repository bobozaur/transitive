use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;

use super::FalliblePathList;
use crate::transitive::TokenizablePath;

/// Path corresponding to a [`#[transitive(try_into(..))`] path.
pub struct TryTransitionInto(FalliblePathList);

impl Parse for TryTransitionInto {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        FalliblePathList::parse(input).map(Self)
    }
}

impl ToTokens for TokenizablePath<'_, &TryTransitionInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let last = self.path.0.path_list.last();
        let second_last = self.path.0.path_list.get(self.path.0.path_list.len() - 2);

        let stmts = self
            .path
            .0
            .path_list
            .iter()
            .take(self.path.0.path_list.len() - 1)
            .map(|ty| quote! {let val: #ty = core::convert::TryFrom::try_from(val)?;});

        let error = self
            .path
            .0
            .error
            .as_ref()
            .map(|e| quote!(#e))
            .unwrap_or_else(|| quote!(<#last as TryFrom<#second_last>>::Error));

        let expanded = quote! {
            impl #impl_generics core::convert::TryFrom<#name #ty_generics> for #last #where_clause {
                type Error = #error;

                fn try_from(val: #name #ty_generics) -> core::result::Result<Self, Self::Error> {
                    #(#stmts)*
                    let val = core::convert::TryFrom::try_from(val)?;
                    Ok(val)
                }
            }
        };

        tokens.extend(expanded);
    }
}
