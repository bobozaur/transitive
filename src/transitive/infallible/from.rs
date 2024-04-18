use std::iter::once;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, punctuated::Punctuated, Path, Token};

use crate::transitive::attr::ParsedAttr;

pub struct TransitiveFrom(Punctuated<Path, Token![,]>);

impl Parse for TransitiveFrom {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Punctuated::parse_terminated(input).map(Self)
    }
}

impl ToTokens for ParsedAttr<'_, &TransitiveFrom> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let first = self.data.0.first();

        let stmts = self
            .data
            .0
            .iter()
            .map(|ty| quote! {let val: #ty = core::convert::From::from(val);})
            .chain(once(quote! {core::convert::From::from(val)}));

        let expanded = quote! {
            impl #impl_generics core::convert::From<#first> for #name #ty_generics #where_clause {
                fn from(val: #first) -> Self {
                    #(#stmts)*
                }
            }
        };

        tokens.extend(expanded);
    }
}
