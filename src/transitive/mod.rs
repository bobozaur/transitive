mod fallible;
mod infallible;

use fallible::{TryTransitionFrom, TryTransitionInto};
use infallible::{TransitionFrom, TransitionInto};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    DeriveInput, Error as SynError, Generics, Ident, MetaList, Result as SynResult, Token,
};

static TOO_FEW_TYPES_ERR_MSG: &str = "at least two types required";

/// The input to the [`crate::Transitive`] derive macro.
pub struct TransitiveInput {
    ident: Ident,
    generics: Generics,
    paths: Vec<TransitionPath>,
}

impl TransitiveInput {
    const ATTR_NAME: &'static str = "transitive";
}

impl Parse for TransitiveInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let DeriveInput {
            attrs,
            ident,
            generics,
            ..
        } = DeriveInput::parse(input)?;

        let fold_fn = |mut vec: Vec<TransitionPath>, res| {
            vec.extend(res?);
            Ok(vec)
        };

        let paths = attrs
            .into_iter()
            .filter(|a| a.path().is_ident(Self::ATTR_NAME))
            .map(|a| a.parse_args_with(Punctuated::<_, Token![,]>::parse_terminated))
            .try_fold::<_, _, SynResult<_>>(Vec::new(), fold_fn)?;

        let output = Self {
            ident,
            generics,
            paths,
        };

        Ok(output)
    }
}

impl ToTokens for TransitiveInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for path in &self.paths {
            TokenizablePath::new(&self.ident, &self.generics, path).to_tokens(tokens);
        }
    }
}

/// Enum representing a path to take when transitioning from one type to another.
pub enum TransitionPath {
    From(TransitionFrom),
    Into(TransitionInto),
    TryFrom(TryTransitionFrom),
    TryInto(TryTransitionInto),
}

impl TransitionPath {
    const FROM: &'static str = "from";
    const INTO: &'static str = "into";
    const TRY_FROM: &'static str = "try_from";
    const TRY_INTO: &'static str = "try_into";
}

impl Parse for TransitionPath {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let MetaList { path, tokens, .. } = MetaList::parse(input)?;
        let tokens = tokens.into();

        match path.require_ident()? {
            ident if ident == Self::FROM => syn::parse(tokens).map(TransitionPath::From),
            ident if ident == Self::INTO => syn::parse(tokens).map(TransitionPath::Into),
            ident if ident == Self::TRY_FROM => syn::parse(tokens).map(TransitionPath::TryFrom),
            ident if ident == Self::TRY_INTO => syn::parse(tokens).map(TransitionPath::TryInto),
            ident => Err(SynError::new(ident.span(), "unknown parameter")),
        }
    }
}

impl ToTokens for TokenizablePath<'_, &TransitionPath> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.path {
            TransitionPath::From(from) => {
                TokenizablePath::new(self.ident, self.generics, from).to_tokens(tokens)
            }
            TransitionPath::Into(into) => {
                TokenizablePath::new(self.ident, self.generics, into).to_tokens(tokens)
            }
            TransitionPath::TryFrom(try_from) => {
                TokenizablePath::new(self.ident, self.generics, try_from).to_tokens(tokens)
            }
            TransitionPath::TryInto(try_into) => {
                TokenizablePath::new(self.ident, self.generics, try_into).to_tokens(tokens)
            }
        }
    }
}

/// Wrapper type that aids in the tokenization of [`TransitionPath`] and its variants.
pub struct TokenizablePath<'a, T> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub path: T,
}

impl<'a, T> TokenizablePath<'a, T> {
    pub fn new(ident: &'a Ident, generics: &'a Generics, path: T) -> Self {
        Self {
            ident,
            generics,
            path,
        }
    }
}
