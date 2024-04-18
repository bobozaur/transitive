mod parsed;

pub use parsed::ParsedAttr;
use syn::{parse::Parse, Error as SynError, MetaList};

use super::{
    fallible::{TransitiveTryFrom, TransitiveTryInto},
    infallible::{TransitiveFrom, TransitiveInto},
};

pub enum TransitiveAttr {
    From(TransitiveFrom),
    Into(TransitiveInto),
    TryFrom(TransitiveTryFrom),
    TryInto(TransitiveTryInto),
}

impl Parse for TransitiveAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let MetaList { path, tokens, .. } = MetaList::parse(input)?;

        let ident = path.require_ident()?;

        if ident == "from" {
            syn::parse::<TransitiveFrom>(tokens.into()).map(TransitiveAttr::From)
        } else if ident == "into" {
            syn::parse::<TransitiveInto>(tokens.into()).map(TransitiveAttr::Into)
        } else if ident == "try_from" {
            syn::parse::<TransitiveTryFrom>(tokens.into()).map(TransitiveAttr::TryFrom)
        } else if ident == "try_into" {
            syn::parse::<TransitiveTryInto>(tokens.into()).map(TransitiveAttr::TryInto)
        } else {
            Err(SynError::new(ident.span(), "unknown parameter"))
        }
    }
}
