use std::iter::once;

use darling::{util::PathList, FromAttributes};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Path;

use crate::transitive::attr::AttrWithIdent;

#[derive(FromAttributes)]
#[darling(attributes(transitive))]
pub struct TransitiveTryFrom {
    try_from: PathList,
    error: Option<Path>,
}

impl ToTokens for AttrWithIdent<'_, &TransitiveTryFrom> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;

        let first = self.data.try_from.first();
        let last = self.data.try_from.last();

        let stmts = self
            .data
            .try_from
            .iter()
            .skip(1)
            .map(|ty| quote! {let val = #ty::try_from(val)?;})
            .chain(once(quote! {let val = #name::try_from(val)?;}));

        let error = self
            .data
            .error
            .as_ref()
            .map(|e| quote!(#e))
            .unwrap_or_else(|| quote!(<#name as TryFrom<#last>>::Error));

        let expanded = quote! {
            impl core::convert::TryFrom<#first> for #name {
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
