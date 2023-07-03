use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

use super::TransitiveAttr;

pub struct AttrWithIdent<'a, T> {
    pub ident: &'a Ident,
    pub data: T,
}

impl<'a, T> AttrWithIdent<'a, T> {
    pub fn new(ident: &'a Ident, data: T) -> Self {
        Self { ident, data }
    }
}

impl ToTokens for AttrWithIdent<'_, TransitiveAttr> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.data {
            TransitiveAttr::From(from) => AttrWithIdent::new(self.ident, from).to_tokens(tokens),
            TransitiveAttr::Into(into) => AttrWithIdent::new(self.ident, into).to_tokens(tokens),
            TransitiveAttr::TryFrom(try_from) => {
                AttrWithIdent::new(self.ident, try_from).to_tokens(tokens)
            }
            TransitiveAttr::TryInto(try_into) => {
                AttrWithIdent::new(self.ident, try_into).to_tokens(tokens)
            }
        }
    }
}
