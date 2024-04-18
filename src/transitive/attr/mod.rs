mod parsed;

use darling::FromAttributes;
use syn::Attribute;

use super::{
    fallible::{TransitiveTryFrom, TransitiveTryInto},
    infallible::{TransitiveFrom, TransitiveInto},
};

pub use parsed::ParsedAttr;

pub enum TransitiveAttr {
    From(TransitiveFrom),
    Into(TransitiveInto),
    TryFrom(TransitiveTryFrom),
    TryInto(TransitiveTryInto),
}

impl TryFrom<Attribute> for TransitiveAttr {
    type Error = darling::Error;

    fn try_from(value: Attribute) -> Result<Self, Self::Error> {
        let Some(ident) = value.path().get_ident() else {
            return Err(darling::Error::missing_field("attribute name"));
        };

        if ident == "transitive_from" {
            TransitiveFrom::from_attributes(&[value]).map(TransitiveAttr::From)
        } else if ident == "transitive_into" {
            TransitiveInto::from_attributes(&[value]).map(TransitiveAttr::Into)
        } else if ident == "transitive_try_from" {
            TransitiveTryFrom::from_attributes(&[value]).map(TransitiveAttr::TryFrom)
        } else if ident == "transitive_try_into" {
            TransitiveTryInto::from_attributes(&[value]).map(TransitiveAttr::TryInto)
        } else {
            Err(darling::Error::unsupported_shape(&ident.to_string()))
        }
    }
}
