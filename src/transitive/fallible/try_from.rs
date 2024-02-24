use std::iter::once;

use darling::{util::PathList, FromAttributes};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Path;

use crate::transitive::attr::ParsedAttr;

#[derive(FromAttributes)]
#[darling(attributes(transitive))]
pub struct TransitiveTryFrom {
    try_from: PathList,
    error: Option<Path>,
}

impl ToTokens for ParsedAttr<'_, &TransitiveTryFrom> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let generic_parameters = self.generic_parameters();
        let simple_generic_parameters = self.simple_generic_parameters();

        let first = self.data.try_from.first();
        let last = self.data.try_from.last();

        let stmts = self
            .data
            .try_from
            .iter()
            .skip(1)
            .map(|ty| quote! {let val: #ty = core::convert::TryFrom::try_from(val)?;})
            .chain(once(
                quote! {let val = core::convert::TryFrom::try_from(val)?;},
            ));

        let error = self
            .data
            .error
            .as_ref()
            .map(|e| quote!(#e))
            .unwrap_or_else(|| quote!(<#name as TryFrom<#last>>::Error));

        let expanded = quote! {
            impl #generic_parameters core::convert::TryFrom<#first> for #name #simple_generic_parameters {
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
