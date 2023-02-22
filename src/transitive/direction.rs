use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, Meta, MetaList, NestedMeta, Result as SynResult,
};

use super::{arg_list_type::ArgListType, FROM, INTO};

pub enum Direction {
    From(ArgListType),
    Into(ArgListType),
}

impl TryFrom<NestedMeta> for Direction {
    type Error = Error;

    fn try_from(value: NestedMeta) -> Result<Self, Self::Error> {
        let NestedMeta::Meta(Meta::List(list)) = value else {
            return Err(Error::new(value.span(), "must provide a direction"))
        };

        list.try_into()
    }
}

impl TryFrom<MetaList> for Direction {
    type Error = Error;

    fn try_from(value: MetaList) -> Result<Self, Self::Error> {
        match value.path.get_ident() {
            Some(i) if i == FROM => Ok(Self::From(value.nested.try_into()?)),
            Some(i) if i == INTO => Ok(Self::Into(value.nested.try_into()?)),
            Some(i) => Err(Error::new(i.span(), format!("unknown argument {i}"))),
            None => Err(Error::new(value.path.span(), "missing direction argument")),
        }
    }
}

/// Implementing [`Parse`] to directly parse the given attribute.
/// The attribute must always have arguments.
///
/// The primary arguments decide the direction: `from()` or `into()`.
/// These contain a types list, optionally encapsulated in
/// `all()`
///
/// The type lists must have at least two arguments, otherwise
/// there's no transition to do.
impl Parse for Direction {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let nested = NestedMeta::parse(input)?;
        nested.try_into()
    }
}
