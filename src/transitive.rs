use darling::{FromAttributes, FromDeriveInput};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, DeriveInput, Ident, Result as SynResult};

use crate::{
    fallible::{TransitiveTryFrom, TransitiveTryInto},
    infallible::{TransitiveFrom, TransitiveInto},
};

#[derive(FromDeriveInput)]
#[darling(forward_attrs(transitive))]
struct DeriveData {
    ident: Ident,
    attrs: Vec<Attribute>,
}

enum TransitiveAttr {
    From(TransitiveFrom),
    Into(TransitiveInto),
    TryFrom(TransitiveTryFrom),
    TryInto(TransitiveTryInto),
}

pub struct TransitiveIdent<'a, T> {
    pub ident: &'a Ident,
    pub data: T,
}

impl<'a, T> TransitiveIdent<'a, T> {
    pub fn new(ident: &'a Ident, data: T) -> Self {
        Self { ident, data }
    }
}

impl FromAttributes for TransitiveAttr {
    fn from_attributes(attrs: &[Attribute]) -> darling::Result<Self> {
        TransitiveFrom::from_attributes(attrs)
            .map(TransitiveAttr::From)
            .or_else(|_| TransitiveInto::from_attributes(attrs).map(TransitiveAttr::Into))
            .or_else(|_| TransitiveTryFrom::from_attributes(attrs).map(TransitiveAttr::TryFrom))
            .or_else(|_| TransitiveTryInto::from_attributes(attrs).map(TransitiveAttr::TryInto))
    }
}

impl ToTokens for TransitiveIdent<'_, TransitiveAttr> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.data {
            TransitiveAttr::From(from) => TransitiveIdent::new(self.ident, from).to_tokens(tokens),
            TransitiveAttr::Into(into) => TransitiveIdent::new(self.ident, into).to_tokens(tokens),
            TransitiveAttr::TryFrom(try_from) => {
                TransitiveIdent::new(self.ident, try_from).to_tokens(tokens)
            }
            TransitiveAttr::TryInto(try_into) => {
                TransitiveIdent::new(self.ident, try_into).to_tokens(tokens)
            }
        }
    }
}

pub fn transitive_impl(input: DeriveInput) -> SynResult<TokenStream> {
    let DeriveData { ident, attrs } = DeriveData::from_derive_input(&input)?;
    let mut output = TokenStream::new();

    for attr in attrs {
        let attr = TransitiveAttr::from_attributes(&[attr])?;
        let attr = TransitiveIdent::new(&ident, attr);
        output.extend(attr.into_token_stream())
    }

    Ok(output)
}
