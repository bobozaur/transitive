mod attr;
mod fallible;
mod infallible;

use darling::{FromAttributes, FromDeriveInput};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, DeriveInput, Ident, Result as SynResult};

use crate::transitive::attr::{AttrWithIdent, TransitiveAttr};

#[derive(FromDeriveInput)]
#[darling(forward_attrs(transitive))]
struct DeriveData {
    ident: Ident,
    attrs: Vec<Attribute>,
}

pub fn transitive_impl(input: DeriveInput) -> SynResult<TokenStream> {
    let DeriveData { ident, attrs } = DeriveData::from_derive_input(&input)?;
    let mut output = TokenStream::new();

    for attr in attrs {
        let attr = TransitiveAttr::from_attributes(&[attr])?;
        let attr = AttrWithIdent::new(&ident, attr);
        output.extend(attr.into_token_stream())
    }

    Ok(output)
}
