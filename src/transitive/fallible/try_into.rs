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
        let generic_parameters = self.generic_parameters();
        let simple_generic_parameters = self.simple_generic_parameters();
        let where_clause = &self.generics.where_clause;

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
            impl #generic_parameters core::convert::TryFrom<#name #simple_generic_parameters> for #last 
            #where_clause {
                type Error = #error;

                fn try_from(val: #name #simple_generic_parameters) -> core::result::Result<Self, Self::Error> {
                    #(#stmts)*
                    let val = core::convert::TryFrom::try_from(val)?;
                    Ok(val)
                }
            }
        };

        tokens.extend(expanded);
    }
}
