use proc_macro2::TokenStream;
use syn::Result as SynResult;
use syn::{punctuated::Punctuated, Attribute, DeriveInput, Ident};

use super::direction::DirectionWrapper;
use super::{arg_handler::ArgHandler, direction::Direction, RawArgList, TRANSITIVE};

pub trait DirectionHandler {
    type IntoHandler: ArgHandler;
    type FromHandler: ArgHandler;
    type Kind: DirectionKind;

    fn handler_into(&self) -> Self::IntoHandler;

    fn handler_from(&self) -> Self::FromHandler;

    fn impl_with_direction(
        &self,
        name: &Ident,
        direction: DirectionWrapper<Self::Kind>,
    ) -> SynResult<Option<TokenStream>> {
        let Some(direction) = direction.into_inner() else {
            return Ok(None)
        };

        match direction {
            Direction::Into(args) => self.handler_into().make_impl(name, args).map(Some),
            Direction::From(args) => self.handler_from().make_impl(name, args).map(Some),
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
                let direction = DirectionWrapper::<Self::Kind>::try_from(nested_meta)?;
                let tokens = self.impl_with_direction(&name, direction)?;

                if let Some(tokens) = tokens {
                    expanded.extend(tokens)
                };
            }
        }

        Ok(expanded)
    }
}

pub trait DirectionKind {
    fn arg_from() -> &'static str;

    fn arg_into() -> &'static str;
}
