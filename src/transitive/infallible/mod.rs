mod from;
mod into;

use std::collections::VecDeque;

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
    intermediate_types: VecDeque<Type>,
    /// Last type in the transitive conversion. ie. `E` in
    /// `#[transitive(from(A, B, C, D, E))]`
    last_type: Type,
}

impl Parse for PathList {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let attr_list = Punctuated::<Type, Token![,]>::parse_terminated(input)?;

        let mut type_list = VecDeque::from_iter(attr_list.iter().cloned());

        static TOO_FEW_TYPES_ERR_MSG: &str =
            "At least two types are required for a transitive conversion";
        let Some(first_type) = type_list.pop_front() else {
            return Err(SynError::new_spanned(attr_list, TOO_FEW_TYPES_ERR_MSG));
        };
        let Some(last_type) = type_list.pop_back() else {
            return Err(SynError::new_spanned(attr_list, TOO_FEW_TYPES_ERR_MSG));
        };

        let output = Self {
            first_type,
            intermediate_types: type_list,
            last_type,
        };

        Ok(output)
    }
}
