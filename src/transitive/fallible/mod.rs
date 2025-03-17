mod try_from;
mod try_into;

use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Error as SynError, Ident, Result as SynResult, Token, Type,
};
pub use try_from::TryTransitionFrom;
pub use try_into::TryTransitionInto;

/// A path list that may contain a custom error type.
pub struct FalliblePathList {
    /// First type in the transitive conversion. ie. `A` in
    /// `#[transitive(try_from(A, B, C, D, E))]`
    first_type: Type,
    /// Intermediate types for the transitive conversion. ie. `[B, .., D]` in
    /// `#[transitive(try_from(A, B, C, D, E))]`
    intermediate_types: Vec<Type>,
    /// Last type in the transitive conversion. ie. `E` in
    /// `#[transitive(try_from(A, B, C, D, E))]`
    last_type: Type,
    error: Option<Type>,
}

impl Parse for FalliblePathList {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let attr_list = Punctuated::<Item, Token![,]>::parse_terminated(input)?;

        let mut attr_list_iter = attr_list.iter().cloned();
        let (first_type, mut last_type) = match (attr_list_iter.next(), attr_list_iter.next()) {
            (Some(Item::Type(first)), Some(Item::Type(last))) => (first, last),
            _ => {
                static TOO_FEW_TYPES_ERR_MSG: &str = "at least two types required";
                return Err(SynError::new(attr_list.span(), TOO_FEW_TYPES_ERR_MSG));
            }
        };

        let mut intermediate_types = Vec::with_capacity(attr_list.len());
        let mut error = None;

        for attr in attr_list_iter {
            match attr {
                Item::Type(ty) if error.is_some() => {
                    let msg = "types not allowed after 'error'";
                    return Err(SynError::new_spanned(ty, msg));
                }
                // Just a regular type path in the conversion path
                Item::Type(ty) => {
                    intermediate_types.push(last_type);
                    last_type = ty;
                }
                Item::Error(err) if error.is_some() => {
                    let msg = "'error' not allowed multiple times";
                    return Err(SynError::new_spanned(err, msg));
                }
                // Custom error, but must check that it's a type path
                Item::Error(err) => error = Some(err),
            }
        }

        let output = Self {
            first_type,
            intermediate_types,
            last_type,
            error,
        };

        Ok(output)
    }
}

/// An item in the parameters list of an attribute.
#[derive(Clone)]
pub enum Item {
    Type(Type),
    Error(Type),
}

impl Parse for Item {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let fork = input.fork();
        // Parse the ident name and the equal sign after it
        let res = fork
            .parse::<Ident>()
            .and_then(|ident| fork.parse::<Token![=]>().map(|_| ident));

        match res {
            // We got an `error = MyType` argument
            Ok(path) if path == "error" => {
                input.advance_to(&fork);
                input.parse().map(Self::Error)
            }
            // Try to parse anything else as a type in the path list
            _ => input.parse().map(Self::Type),
        }
    }
}

impl quote::ToTokens for Item {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Type(ty) | Self::Error(ty) => ty.to_tokens(tokens),
        }
    }
}
