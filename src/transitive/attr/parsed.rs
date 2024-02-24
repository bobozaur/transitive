use proc_macro2::TokenStream;
use quote::ToTokens;
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
