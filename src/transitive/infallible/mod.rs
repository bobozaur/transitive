mod from;
mod into;

pub use from::TransitionFrom;
pub use into::TransitionInto;
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult, Type,
};

use crate::transitive::AtLeastTwoTypes;

struct TypeList {
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

impl Parse for TypeList {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let AtLeastTwoTypes {
            first_type,
            second_type: mut last_type,
            remaining,
        } = AtLeastTwoTypes::parse(input)?;

        let intermediate_types = remaining
            .map(|ty| std::mem::replace(&mut last_type, ty))
            .collect();

        let output = Self {
            first_type,
            intermediate_types,
            last_type,
        };

        Ok(output)
    }
}
