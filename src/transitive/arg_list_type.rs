use syn::{spanned::Spanned, Error, Meta, MetaList, NestedMeta, Result as SynResult};

use super::{minimal_args::MinimalAttrArgs, RawArgList, ALL};

/// Enum diferentiating between the simple or enhance
/// transitive behavior.
pub enum ArgListType {
    Simple(MinimalAttrArgs),
    All(MinimalAttrArgs),
}

/// Conversion between [`RawArgList`] and [`ArgListType`],
/// used to determine how to process the arguments provided.
/// The attribute must always have arguments.
///
/// If there's only one argument, we assume that is the `all()`
/// argument which contains a list in itself, annotating to use
/// the enhanced behavior (multiple sources or targets).
///
/// Otherwise, consider the arguments as a list of type to transition
/// through from only one source to only one target.
impl TryFrom<RawArgList> for ArgListType {
    type Error = Error;

    fn try_from(value: RawArgList) -> SynResult<Self> {
        match value.len() {
            0 => Err(Error::new(value.span(), "missing arguments")),
            1 => Ok(value.into_iter().next().unwrap().try_into()?),
            _ => Ok(Self::Simple(value.try_into()?)),
        }
    }
}

impl TryFrom<NestedMeta> for ArgListType {
    type Error = Error;

    fn try_from(value: NestedMeta) -> Result<Self, Self::Error> {
        let NestedMeta::Meta(Meta::List(list)) = value else {
            return Err(Error::new(value.span(), "must provide a list of types"))
        };

        list.try_into()
    }
}

impl TryFrom<MetaList> for ArgListType {
    type Error = Error;

    fn try_from(value: MetaList) -> Result<Self, Self::Error> {
        if value.path.is_ident(ALL) {
            Ok(Self::All(value.nested.try_into()?))
        } else {
            Err(Error::new(value.path.span(), "unknown argument"))
        }
    }
}
