pub mod from;
pub mod into;
pub mod try_from;
pub mod try_into;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, DeriveInput, Error, Ident, Meta, MetaList, NestedMeta, Path, Result as SynResult,
    Token,
};

type ArgList = Punctuated<NestedMeta, Token![,]>;

const TRANSITIVE: &str = "transitive";
const ALL: &str = "all";

/// Parse input and processes attributes with the provided parameters.
pub fn transitive_impl<F, G>(
    input: DeriveInput,
    conv_func: &TokenStream,
    ts_maker: &F,
    raise_error: bool,
    impl_creator: &G,
) -> SynResult<TokenStream>
where
    F: Fn(TokenStream, &Ident, Path, Path) -> TokenStream,
    G: Fn(&Ident, &Path, &Path) -> TokenStream,
{
    let name = input.ident;
    let mut expanded = TokenStream::new();

    let iter = input.attrs.into_iter().filter_map(|attr| {
        process_attr(&name, attr, conv_func, ts_maker, raise_error, impl_creator)
    });

    for token_stream in iter {
        expanded.extend(token_stream?);
    }

    Ok(expanded)
}

/// Determines the attribute kind and processes it accordingly.
fn process_attr<F, G>(
    name: &Ident,
    attr: Attribute,
    conv_func: &TokenStream,
    ts_maker: &F,
    raise_error: bool,
    impl_creator: &G,
) -> Option<SynResult<TokenStream>>
where
    F: Fn(TokenStream, &Ident, Path, Path) -> TokenStream,
    G: Fn(&Ident, &Path, &Path) -> TokenStream,
{
    if attr.path.is_ident(TRANSITIVE) {
        let result = match attr.parse_args_with(ArgsListType::parse) {
            Ok(ArgsListType::Simple(s)) => {
                process_transitive(name, s, conv_func, ts_maker, raise_error)
            }
            Ok(ArgsListType::All(s)) => process_transitive_all(name, s, impl_creator),
            Err(e) => Err(e),
        };

        Some(result)
    } else {
        None
    }
}

/// Processes an argument list considering regular behavior
/// of implementing the trait only between source type and target type.
fn process_transitive<F>(
    name: &Ident,
    arg_list: ArgList,
    conv_func: &TokenStream,
    ts_maker: &F,
    raise_err: bool,
) -> SynResult<TokenStream>
where
    F: Fn(TokenStream, &Ident, Path, Path) -> TokenStream,
{
    let MinimalAttrArgs {
        first,
        mut last,
        iter,
    } = validate_arg_list(arg_list)?;

    let raise = if raise_err {
        quote! {?}
    } else {
        TokenStream::new()
    };

    // Create the buffer and store the minimum amount of statements.
    let mut stmts = TokenStream::new();
    stmts.extend(quote! {let interm = #first::#conv_func(val)#raise;});
    stmts.extend(quote! {let interm = #last::#conv_func(interm)#raise;});

    // Store other statements, if any
    for param in iter {
        last = param?;
        stmts.extend(quote! {let interm = #last::#conv_func(interm)#raise;});
    }

    // Generate code
    let expanded = ts_maker(stmts, name, first, last);
    Ok(expanded)
}

/// Processes an argument list considering the enhanced behavior
/// of implementing the trait between all transitions from either
/// one source and multiple targets or multiple targets and once source,
/// depending on the trait.
fn process_transitive_all<F>(
    name: &Ident,
    arg_list: ArgList,
    impl_creator: F,
) -> SynResult<TokenStream>
where
    F: Fn(&Ident, &Path, &Path) -> TokenStream,
{
    let MinimalAttrArgs {
        mut first,
        mut last,
        iter,
    } = validate_arg_list(arg_list)?;

    // Create the buffer and store the first impl.
    let mut impls = TokenStream::new();
    impls.extend(impl_creator(name, &first, &last));

    // Create and store other impls, if any
    for param in iter {
        first = last;
        last = param?;
        impls.extend(impl_creator(name, &first, &last));
    }

    Ok(impls)
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

/// Enum diferentiating between the simple or enhance
/// transitive behavior.
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

/// Implementing [`Parse`] to directly parse the given attribute.
/// The attribute must always have arguments.
///
/// If there's only one argument, we assume that is the `all()`
/// argument which contains a list in itself, annotating to use
/// the enhanced behavior (multiple sources or targets).
///
/// Otherwise, consider the arguments as a list of type to transition
/// through from only one source to only one target.
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
