use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Generics, Ident};

use super::TransitiveAttr;

pub struct ParsedAttr<'a, T> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub data: T,
}

impl<'a, T> ParsedAttr<'a, T> {
    pub fn new(ident: &'a Ident, generics: &'a Generics, data: T) -> Self {
        Self {
            ident,
            generics,
            data,
        }
    }

    pub fn generic_parameters(&self) -> TokenStream {
        let Generics {
            lt_token,
            params,
            gt_token,
            ..
        } = self.generics;

        quote! {#lt_token #params #gt_token}
    }

    /// Note:
    /// Must ask for key value pairs in `with()`
    /// and map them against the provided generics.
    ///
    /// Then replace the ones with the provided values while
    /// preserving the ones without values.'
    /// Should also replace them in their trait bounds.

    pub fn simple_generic_parameters(&self) -> TokenStream {
        let Generics {
            lt_token,
            params,
            gt_token,
            ..
        } = self.generics;

        let iter = params.iter().map(|v| match v {
            syn::GenericParam::Lifetime(v) => {
                let tt = &v.lifetime;
                quote!(#tt)
            }
            syn::GenericParam::Type(v) => {
                let tt = &v.ident;
                quote!(#tt)
            }
            syn::GenericParam::Const(v) => {
                let tt = &v.ident;
                quote!(#tt)
            }
        });

        quote! {#lt_token #(#iter),* #gt_token}
    }
}

impl ToTokens for ParsedAttr<'_, TransitiveAttr> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.data {
            TransitiveAttr::From(from) => {
                ParsedAttr::new(self.ident, self.generics, from).to_tokens(tokens)
            }
            TransitiveAttr::Into(into) => {
                ParsedAttr::new(self.ident, self.generics, into).to_tokens(tokens)
            }
            TransitiveAttr::TryFrom(try_from) => {
                ParsedAttr::new(self.ident, self.generics, try_from).to_tokens(tokens)
            }
            TransitiveAttr::TryInto(try_into) => {
                ParsedAttr::new(self.ident, self.generics, try_into).to_tokens(tokens)
            }
        }
    }
}
