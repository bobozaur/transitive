mod into;
mod from;
mod try_into;
mod try_from;

use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, DeriveInput, Error, Ident, Meta, MetaList, NestedMeta, Path, Result as SynResult,
    Token,
};

pub use into::into_process_attr;
pub use from::from_process_attr;
pub use try_into::try_into_process_attr;
pub use try_from::try_from_process_attr;

type ArgList = Punctuated<NestedMeta, Token![,]>;

const TRANSITIVE: &str = "transitive";
const ALL: &str = "all";

/// Minimal arguments the transitive attributes should have,
/// along with an iterator of the remaining items
struct MinimalAttrArgs<I>
where
    I: Iterator<Item = SynResult<Path>>,
{
    first: Path,
    last: Path,
    iter: I,
}

enum ArgsListType {
    Simple(ArgList),
    All(ArgList),
}

impl TryFrom<NestedMeta> for ArgsListType {
    type Error = Error;

    fn try_from(value: NestedMeta) -> Result<Self, Self::Error> {
        let NestedMeta::Meta(Meta::List(list)) = value else {
            return Err(Error::new(value.span(), "must provide a list of types"))
        };

        list.try_into()
    }
}

impl TryFrom<MetaList> for ArgsListType {
    type Error = Error;

    fn try_from(value: MetaList) -> Result<Self, Self::Error> {
        if value.path.is_ident(ALL) {
            Ok(Self::All(value.nested))
        } else {
            Err(Error::new(value.path.span(), "unknown argument"))
        }
    }
}

impl Parse for ArgsListType {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let list: Punctuated<NestedMeta, _> = Punctuated::parse_terminated(input)?;
        match list.len() {
            0 => Err(Error::new(input.span(), "missing arguments")),
            1 => Ok(list.into_iter().next().unwrap().try_into()?),
            _ => Ok(Self::Simple(list)),
        }
    }
}

/// Parse input and processes attributes with the provided closure.
pub fn transitive_impl<F>(input: DeriveInput, process_attr: F) -> SynResult<TokenStream>
where
    F: Fn(&Ident, Attribute) -> Option<SynResult<TokenStream>>,
{
    let name = input.ident;
    let mut expanded = TokenStream::new();
    for token_stream in input
        .attrs
        .into_iter()
        .filter_map(|attr| process_attr(&name, attr))
    {
        expanded.extend(token_stream?);
    }
    Ok(expanded)
}

/// Checks that the attribute was given the minimum needed arguments
/// and returns the arguments as a [`MinimalAttrArgs`] type.
fn validate_arg_list(
    arg_list: ArgList,
) -> SynResult<MinimalAttrArgs<impl Iterator<Item = SynResult<Path>>>> {
    // Save the span in case we issue errors.
    // Consuming the attribute arguments prevents us from doing that later.
    let span = arg_list.span();

    // Parse arguments and create an iterator of [`Path`] (types) items.
    let mut iter = arg_list.into_iter().map(is_type_path);

    // Ensure we were provided with at least two elements.
    let (first, last) = match (iter.next(), iter.next()) {
        (Some(first), Some(last)) => Ok((first?, last?)),
        _ => Err(Error::new(span, "at least two parameters needed")),
    }?;

    let output = MinimalAttrArgs { first, last, iter };

    Ok(output)
}

/// Ensures we only accept types, not literals, integers or anything like that.
fn is_type_path(param: NestedMeta) -> SynResult<Path> {
    match param {
        NestedMeta::Meta(Meta::Path(p)) => Ok(p),
        _ => Err(Error::new(param.span(), "only type paths accepted")),
    }
}
