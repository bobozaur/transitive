use darling::{util::PathList, FromAttributes};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Path;

use crate::transitive::attr::AttrWithIdent;

#[derive(FromAttributes)]
#[darling(attributes(transitive))]
pub struct TransitiveTryInto {
    try_into: PathList,
    error: Option<Path>,
}

impl ToTokens for AttrWithIdent<'_, &TransitiveTryInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;

        let last = self.data.try_into.last();
        let second_last = self.data.try_into.iter().nth(self.data.try_into.len() - 2);

        let stmts = self
            .data
            .try_into
            .iter()
            .map(|ty| quote! {let val = #ty::try_from(val)?;});

        let error = self
            .data
            .error
            .as_ref()
            .map(|e| quote!(#e))
            .unwrap_or_else(|| quote!(<#last as TryFrom<#second_last>>::Error));

        let expanded = quote! {
            impl core::convert::TryFrom<#name> for #last {
                type Error = #error;

                fn try_from(val: #name) -> core::result::Result<Self, Self::Error> {
                    #(#stmts)*
                    Ok(val)
                }
            }
        };

        tokens.extend(expanded);
    }
}
