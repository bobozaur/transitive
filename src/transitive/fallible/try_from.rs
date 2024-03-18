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
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
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
