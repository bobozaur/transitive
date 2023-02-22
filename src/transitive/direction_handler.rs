use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, Attribute, DeriveInput, Ident, Result as SynResult};

use super::{arg_handler::ArgHandler, Direction, RawArgList, TRANSITIVE};

pub trait DirectionHandler {
    type IntoHandler: ArgHandler;
    type FromHandler: ArgHandler;

    fn make_into_handler(&self) -> Self::IntoHandler;

    fn make_from_handler(&self) -> Self::FromHandler;

    fn impl_with_direction(&self, name: &Ident, direction: Direction) -> SynResult<TokenStream> {
        match direction {
            Direction::Into(args) => self.make_into_handler().make_impl(name, args),
            Direction::From(args) => self.make_from_handler().make_impl(name, args),
        }
    }

    /// Determines the attribute kind and processes it accordingly.
    fn parse_transitive_attr(attr: Attribute) -> Option<SynResult<RawArgList>> {
        if attr.path.is_ident(TRANSITIVE) {
            Some(attr.parse_args_with(Punctuated::parse_terminated))
        } else {
            None
        }
    }

    /// Parse input and processes attributes with the provided parameters.
    fn generate_tokens(&self, input: DeriveInput) -> SynResult<TokenStream> {
        let name = input.ident;
        let mut expanded = TokenStream::new();

        let attr_iter = input
            .attrs
            .into_iter()
            .filter_map(Self::parse_transitive_attr);

        for meta_iter in attr_iter {
            for nested_meta in meta_iter? {
                let direction = Direction::try_from(nested_meta)?;
                let tokens = self.impl_with_direction(&name, direction)?;
                expanded.extend(tokens);
            }
        }

        Ok(expanded)
    }
}
