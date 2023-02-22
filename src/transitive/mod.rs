mod arg_handler;
pub mod direction_handler;
pub mod fallible;
pub mod infallible;

use std::iter::Map;

use syn::{
    parse::{Parse, ParseStream},
    punctuated::{IntoIter, Punctuated},
    spanned::Spanned,
    Error, Meta, MetaList, NestedMeta, Path, Result as SynResult, Token,
};

type RawArgList = Punctuated<NestedMeta, Token![,]>;
type ArgsIter = Map<IntoIter<NestedMeta>, ArgMapFn>;
type ArgMapFn = fn(NestedMeta) -> SynResult<Path>;

const TRANSITIVE: &str = "transitive";
const ALL: &str = "all";
const FROM: &str = "from";
const INTO: &str = "into";
const TRY_INTO: &str = "try_into";
const TRY_FROM: &str = "try_from";

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

/// Minimal arguments the transitive attributes should have,
/// along with an iterator of the remaining items
pub struct MinimalAttrArgs {
    first: Path,
    last: Path,
    iter: ArgsIter,
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
