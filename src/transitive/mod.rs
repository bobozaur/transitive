mod attr;
mod fallible;
mod infallible;

use darling::{FromAttributes, FromDeriveInput};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, DeriveInput, Generics, Ident, Result as SynResult};

use crate::transitive::attr::{ParsedAttr, TransitiveAttr};

#[derive(FromDeriveInput)]
#[darling(forward_attrs(transitive))]
struct DeriveData {
    ident: Ident,
    generics: Generics,
    attrs: Vec<Attribute>,
}

pub fn transitive_impl(input: DeriveInput) -> SynResult<TokenStream> {
    let DeriveData {
        ident,
        generics,
        attrs,
    } = DeriveData::from_derive_input(&input)?;
    let mut output = TokenStream::new();

    for attr in attrs {
        let attr = TransitiveAttr::from_attributes(&[attr])?;
        let attr = ParsedAttr::new(&ident, &generics, attr);
        output.extend(attr.into_token_stream())
    }

    Ok(output)
}
