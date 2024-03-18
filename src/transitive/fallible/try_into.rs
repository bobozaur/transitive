use darling::{util::PathList, FromAttributes};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Path;

use crate::transitive::attr::ParsedAttr;

#[derive(FromAttributes)]
#[darling(attributes(transitive))]
pub struct TransitiveTryInto {
    try_into: PathList,
    error: Option<Path>,
}

impl ToTokens for ParsedAttr<'_, &TransitiveTryInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let last = self.data.try_into.last();
        let second_last = self.data.try_into.iter().nth(self.data.try_into.len() - 2);

        let stmts = self
            .data
            .try_into
            .iter()
            .take(self.data.try_into.len() - 1)
            .map(|ty| quote! {let val: #ty = core::convert::TryFrom::try_from(val)?;});

        let error = self
            .data
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
