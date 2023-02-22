use syn::{spanned::Spanned, Error, Meta, NestedMeta, Path, Result as SynResult};

use super::{ArgMapFn, ArgsIter, RawArgList};

/// Minimal arguments the transitive attributes should have,
/// along with an iterator of the remaining items
pub struct MinimalAttrArgs {
    pub first: Path,
    pub last: Path,
    pub iter: ArgsIter,
}

impl MinimalAttrArgs {
    /// Ensures we only accept types, not literals, integers or anything like that.
    fn is_type_path(param: NestedMeta) -> SynResult<Path> {
        match param {
            NestedMeta::Meta(Meta::Path(p)) => Ok(p),
            _ => Err(Error::new(param.span(), "only type paths accepted")),
        }
    }
}

/// Checks that the attribute was given the minimum needed arguments
/// and returns the arguments as a [`MinimalAttrArgs`] type.
impl TryFrom<RawArgList> for MinimalAttrArgs {
    type Error = Error;

    fn try_from(value: RawArgList) -> Result<Self, Self::Error> {
        // Save the span in case we issue errors.
        // Consuming the attribute arguments prevents us from doing that later.
        let span = value.span();

        // Parse arguments and create an iterator of [`Path`] (types) items.
        let mut iter = value.into_iter().map(Self::is_type_path as ArgMapFn);

        // Ensure we were provided with at least two elements.
        let (first, last) = match (iter.next(), iter.next()) {
            (Some(first), Some(last)) => Ok((first?, last?)),
            _ => Err(Error::new(span, "at least two parameters needed")),
        }?;

        Ok(MinimalAttrArgs { first, last, iter })
    }
}
