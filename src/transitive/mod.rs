mod fallible;
mod infallible;

use fallible::{TransitiveTryFrom, TransitiveTryInto};
use infallible::{TransitiveFrom, TransitiveInto};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    DeriveInput, Error as SynError, Generics, Ident, MetaList, Result as SynResult, Token,
};

pub struct TransitiveInput {
    ident: Ident,
    generics: Generics,
    attrs: Vec<TransitiveAttr>,
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

        let fold_fn = |mut vec: Vec<TransitiveAttr>, res| {
            vec.extend(res?);
            Ok(vec)
        };

        let attrs = attrs
            .into_iter()
            .filter(|a| a.path().is_ident(Self::ATTR_NAME))
            .map(|a| a.parse_args_with(Punctuated::<_, Token![,]>::parse_terminated))
            .try_fold::<_, _, SynResult<_>>(Vec::new(), fold_fn)?;

        let output = Self {
            ident,
            generics,
            attrs,
        };

        Ok(output)
    }
}

impl ToTokens for TransitiveInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attr in &self.attrs {
            TokenizableAttr::new(&self.ident, &self.generics, attr).to_tokens(tokens);
        }
    }
}

pub enum TransitiveAttr {
    From(TransitiveFrom),
    Into(TransitiveInto),
    TryFrom(TransitiveTryFrom),
    TryInto(TransitiveTryInto),
}

impl TransitiveAttr {
    const FROM: &'static str = "from";
    const INTO: &'static str = "into";
    const TRY_FROM: &'static str = "try_from";
    const TRY_INTO: &'static str = "try_into";
}

impl Parse for TransitiveAttr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let MetaList { path, tokens, .. } = MetaList::parse(input)?;
        let tokens = tokens.into();

        match path.require_ident()? {
            ident if ident == Self::FROM => syn::parse(tokens).map(TransitiveAttr::From),
            ident if ident == Self::INTO => syn::parse(tokens).map(TransitiveAttr::Into),
            ident if ident == Self::TRY_FROM => syn::parse(tokens).map(TransitiveAttr::TryFrom),
            ident if ident == Self::TRY_INTO => syn::parse(tokens).map(TransitiveAttr::TryInto),
            ident => Err(SynError::new(ident.span(), "unknown parameter")),
        }
    }
}

impl ToTokens for TokenizableAttr<'_, &TransitiveAttr> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.attr {
            TransitiveAttr::From(from) => {
                TokenizableAttr::new(self.ident, self.generics, from).to_tokens(tokens)
            }
            TransitiveAttr::Into(into) => {
                TokenizableAttr::new(self.ident, self.generics, into).to_tokens(tokens)
            }
            TransitiveAttr::TryFrom(try_from) => {
                TokenizableAttr::new(self.ident, self.generics, try_from).to_tokens(tokens)
            }
            TransitiveAttr::TryInto(try_into) => {
                TokenizableAttr::new(self.ident, self.generics, try_into).to_tokens(tokens)
            }
        }
    }
}

/// Wrapper type that aids in the tokenization of [`TransitiveAttr`] and its variants.
pub struct TokenizableAttr<'a, T> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub attr: T,
}

impl<'a, T> TokenizableAttr<'a, T> {
    pub fn new(ident: &'a Ident, generics: &'a Generics, attr: T) -> Self {
        Self {
            ident,
            generics,
            attr,
        }
    }
}
