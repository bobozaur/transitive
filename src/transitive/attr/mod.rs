mod idented;

use darling::FromAttributes;
use syn::Attribute;

use super::{
    fallible::{TransitiveTryFrom, TransitiveTryInto},
    infallible::{TransitiveFrom, TransitiveInto},
};

pub use idented::AttrWithIdent;

pub enum TransitiveAttr {
    From(TransitiveFrom),
    Into(TransitiveInto),
    TryFrom(TransitiveTryFrom),
    TryInto(TransitiveTryInto),
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
