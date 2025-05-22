mod from;
mod into;

use std::mem;

pub use from::TransitionFrom;
pub use into::TransitionInto;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error as SynError, Result as SynResult, Token, Type,
};

use crate::transitive::TOO_FEW_TYPES_ERR_MSG;

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
        let error_span = input.span();
        let attr_list = Punctuated::<Type, Token![,]>::parse_terminated(input)?;

        let mut attr_list_iter = attr_list.into_iter();
        let (first_type, mut last_type) = match (attr_list_iter.next(), attr_list_iter.next()) {
            (Some(first_type), Some(last_type)) => (first_type, last_type),
            _ => return Err(SynError::new(error_span, TOO_FEW_TYPES_ERR_MSG)),
        };
        let intermediate_types = attr_list_iter
            .map(|ty| mem::replace(&mut last_type, ty))
            .collect();

        let output = Self {
            first_type,
            intermediate_types,
            last_type,
        };

        Ok(output)
    }
}
