mod try_from;
mod try_into;

use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    Error as SynError, Ident, Result as SynResult, Token, Type,
};
pub use try_from::TryTransitionFrom;
pub use try_into::TryTransitionInto;

use crate::transitive::AtLeastTwoTypes;

/// A path list that may contain a custom error type.
struct FallibleTypeList {
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

impl Parse for FallibleTypeList {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let AtLeastTwoTypes {
            first_type,
            second_type: mut last_type,
            remaining,
        } = AtLeastTwoTypes::parse(input)?;

        let mut intermediate_types = Vec::with_capacity(remaining.len());
        let mut error = None;

        for attr in remaining {
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
enum Item {
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

impl From<Item> for Option<Type> {
    fn from(value: Item) -> Self {
        match value {
            Item::Type(ty) => Some(ty),
            Item::Error(_) => None,
        }
    }
}
