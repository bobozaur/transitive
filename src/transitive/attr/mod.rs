mod parsed;

use darling::FromAttributes;
use quote::ToTokens;
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
        match value.path().to_token_stream().to_string().as_str() {
            "transitive_from" => {
                TransitiveFrom::from_attributes(&[value]).map(TransitiveAttr::From)
            }
            "transitive_into" => {
                TransitiveInto::from_attributes(&[value]).map(TransitiveAttr::Into)
            }
            "transitive_try_from" => {
                TransitiveTryFrom::from_attributes(&[value]).map(TransitiveAttr::TryFrom)
            }
            "transitive_try_into" => {
                TransitiveTryInto::from_attributes(&[value]).map(TransitiveAttr::TryInto)
            }
            p => Err(darling::Error::unsupported_shape(p)),
        }
    }
}
