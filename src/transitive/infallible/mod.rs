mod from;
mod into;

pub use from::TransitionFrom;
pub use into::TransitionInto;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error as SynError, Result as SynResult, Token, Type,
};

pub struct PathList {
    /// First type in the transitive conversion. ie. `A` in
    /// `#[transitive(from(A, B, C, D, E))]`
    first_type: Type,
    /// Intermediate types for the transitive conversion. ie. `[B, .., D]` in
    /// `#[transitive(from(A, B, C, D, E))]`
    intermediate_types: Vec<Type>,
    /// Last type in the transitive conversion. ie. `E` in
    /// `#[transitive(from(A, B, C, D, E))]`
    last_type: Type,
}

impl Parse for PathList {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let attr_list = Punctuated::<Type, Token![,]>::parse_terminated(input)?;

        let mut attr_list_iter = attr_list.iter().cloned();
        let (first_type, mut last_type) = match (attr_list_iter.next(), attr_list_iter.next()) {
            (Some(first_type), Some(last_type)) => (first_type, last_type),
            _ => {
                static TOO_FEW_TYPES_ERR_MSG: &str = "at least two types required";
                return Err(SynError::new_spanned(attr_list, TOO_FEW_TYPES_ERR_MSG));
            }
        };
        let mut intermediate_types = Vec::with_capacity(attr_list.len());
        for ty in attr_list_iter {
            intermediate_types.push(last_type);
            last_type = ty;
        }

        let output = Self {
            first_type,
            intermediate_types,
            last_type,
        };

        Ok(output)
    }
}
