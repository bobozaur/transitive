use std::iter::once;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;

use super::FalliblePathList;
use crate::transitive::attr::ParsedAttr;

pub struct TransitiveTryFrom(FalliblePathList);

impl Parse for TransitiveTryFrom {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        FalliblePathList::parse(input).map(Self)
    }
}

impl ToTokens for ParsedAttr<'_, &TransitiveTryFrom> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let first = self.data.0.path_list.first();
        let last = self.data.0.path_list.last();

        let stmts = self
            .data
            .0
            .path_list
            .iter()
            .skip(1)
            .map(|ty| quote! {let val: #ty = core::convert::TryFrom::try_from(val)?;})
            .chain(once(
                quote! {let val = core::convert::TryFrom::try_from(val)?;},
            ));

        let error = self
            .data
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
