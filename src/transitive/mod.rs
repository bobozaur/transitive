mod attr;
mod fallible;
mod infallible;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Attribute, DeriveInput, Error as SynError, Generics, Ident, Meta, MetaList, MetaNameValue,
    Path, Result as SynResult,
};

use crate::transitive::attr::{ParsedAttr, TransitiveAttr};

const ATTR_NAME: &str = "transitive";

pub fn transitive_impl(input: DeriveInput) -> SynResult<TokenStream> {
    let DeriveInput {
        ident,
        generics,
        attrs,
        ..
    } = input;

    attrs
        .into_iter()
        .map(|attr| process_attribute(attr, &ident, &generics))
        .collect()
}

fn process_attribute(
    attr: Attribute,
    ident: &Ident,
    generics: &Generics,
) -> SynResult<TokenStream> {
    let tokens = match attr.meta {
        Meta::Path(path) | Meta::NameValue(MetaNameValue { path, .. }) => return wrong_meta(path),
        Meta::List(MetaList { path, tokens, .. }) => match path.get_ident() {
            Some(i) if i == ATTR_NAME => tokens,
            _ => return Ok(TokenStream::new()),
        },
    };

    let attr = syn::parse::<TransitiveAttr>(tokens.into())?;
    let attr = ParsedAttr::new(ident, generics, attr);
    Ok(attr.into_token_stream())
}

fn wrong_meta(path: Path) -> SynResult<TokenStream> {
    let ident = match path.get_ident() {
        Some(i) if i == ATTR_NAME => i,
        _ => return Ok(TokenStream::new()),
    };

    Err(SynError::new(
        ident.span(),
        "only list attributes are allowed",
    ))
}
