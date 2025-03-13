mod try_from;
mod try_into;

use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    punctuated::Punctuated,
    Error as SynError, Ident, Result as SynResult, Token, Type,
};
pub use try_from::TryTransitionFrom;
pub use try_into::TryTransitionInto;

/// A path list that may contain a custom error type.
pub struct FalliblePathList {
    type_list: Vec<Type>,
    error: Option<Type>,
}

impl Parse for FalliblePathList {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let attr_list = Punctuated::<AttributeValue, Token![,]>::parse_terminated(input)?;

        let mut type_list = Vec::with_capacity(attr_list.len());
        let mut error = None;

        for attr in attr_list {
            match attr {
                AttributeValue::Type(ty) if error.is_some() => {
                    let msg = "types not allowed after 'error' or 'bounds'";
                    return Err(SynError::new_spanned(ty, msg));
                }
                // Just a regular type path in the conversion path
                AttributeValue::Type(ty) => type_list.push(ty),
                AttributeValue::Error(err) if error.is_some() => {
                    let msg = "'error' not allowed multiple times";
                    return Err(SynError::new_spanned(err, msg));
                }
                // Custom error, but must check that it's a type path
                AttributeValue::Error(err) => error = Some(err),
            }
        }

        let output = Self { type_list, error };

        Ok(output)
    }
}

pub enum AttributeValue {
    Type(Type),
    Error(Type),
}

impl Parse for AttributeValue {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let fork = input.fork();
        // Parse the ident name and the equal sign after it
        let res = fork
            .parse::<Ident>()
            .and_then(|ident| fork.parse::<Token![=]>().map(|_| ident));

        match res {
            Ok(path) if path == "error" => {
                input.advance_to(&fork);
                input.parse().map(Self::Error)
            }
            _ => input.parse().map(Self::Type),
        }
    }
}
