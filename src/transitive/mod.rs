mod fallible;
mod infallible;

use fallible::{TryTransitionFrom, TryTransitionInto};
use infallible::{TransitionFrom, TransitionInto};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    DeriveInput, Error as SynError, Generics, Ident, ImplGenerics, MetaList, Result as SynResult,
    Token, Type, TypeGenerics, WhereClause,
};

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
enum TransitionPath {
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
struct TokenizablePath<'a, T> {
    ident: &'a Ident,
    generics: &'a Generics,
    path: T,
}

impl<'a, T> TokenizablePath<'a, T> {
    fn new(ident: &'a Ident, generics: &'a Generics, path: T) -> Self {
        Self {
            ident,
            generics,
            path,
        }
    }
}

/// Parsing helper that guarantees that there are at least two [`Type`] items in the list.
///
/// ```compile_fail
/// use transitive::Transitive;
///
/// struct A;
/// #[derive(Transitive)]
/// #[transitive(from(A))] // fails to compile, list too short
/// struct B;
///
/// impl From<A> for B {
///     fn from(_: A) -> B {
///         Self
///     }
/// }
/// ```
struct AtLeastTwoTypes<T> {
    /// First type in the list.
    first_type: Type,
    /// Second type in the list.
    second_type: Type,
    /// Remaining items in the input.
    /// These are NOT guaranteed to be types!
    remaining: syn::punctuated::IntoIter<T>,
}

impl<T> Parse for AtLeastTwoTypes<T>
where
    T: Parse,
    Option<Type>: From<T>,
{
    fn parse(input: ParseStream) -> SynResult<Self> {
        let error_span = input.span();

        let mut remaining = Punctuated::<T, Token![,]>::parse_terminated(input)?.into_iter();
        let first_opt = remaining.next().and_then(From::from);
        let second_opt = remaining.next().and_then(From::from);

        let (first_type, second_type) = match (first_opt, second_opt) {
            (Some(first_type), Some(last_type)) => (first_type, last_type),
            _ => return Err(SynError::new(error_span, "at least two types required")),
        };

        let output = Self {
            first_type,
            second_type,
            remaining,
        };

        Ok(output)
    }
}

/// Inserts a compile time check that the first and last types are not the same.
///
/// ```compile_fail
/// use transitive::Transitive;
///
/// struct A;
/// #[derive(Transitive)]
/// #[transitive(try_into(A, A))] // fails to compile, first and last types are equal
/// struct B;
///
/// impl TryFrom<B> for A {
///     type Error = ();
///
///     fn try_from(_: B) -> Result<Self, Self::Error> {
///         Ok(Self)
///     }
/// }
/// ```
fn distinct_types_check<'a>(
    left: &Type,
    right: &Type,
    derived: &Ident,
    impl_generics: &ImplGenerics<'a>,
    ty_generics: &TypeGenerics<'a>,
    where_clause: Option<&WhereClause>,
) -> TokenStream {
    let turbofish = ty_generics.as_turbofish();

    // We first declare an inherent constant on the wrapper type when we specifically use the
    // `left` type.
    //
    // We use `(left, derived)` as the generic type for [`Checker`] because `left` might use
    // generics of `derived`, but we do not have generic components ([`ImplGenerics`],
    // [`TypeGenerics`], etc.) for other types other than `derived`, which is the derived type.
    let checker = quote! {
        struct Checker<T>(core::marker::PhantomData<fn() -> T>);

        impl #impl_generics Checker<(#left, #derived #ty_generics)> #where_clause {
            const FLAG: bool = false;
        }
    };

    // We now declare a trait with a constant that has the same name as the inherent one and a
    // blanket impl.
    //
    // The `left` type will also get the trait constant, but the inherent constant has priority.
    // Because of that, if the left and right types are the same then the inherent constant gets
    // used in the assertion twice.
    let flagged = quote! {
        #checker

        trait Flagged {
            const FLAG: bool;
        }

        impl<T> Flagged for Checker<T> {
            const FLAG: bool = true;
        }
    };

    // The actual assert resides in a trait constant to allow the usage of generics in the
    // const context. A regular const declaration would not allow the usage of generics from the
    // outer scope.
    //
    // The assert merely does an XOR on the checker flags. If the types are different, then
    // one flag will be the inherent constant while the other will come from the trait blanket
    // impl.
    //
    // We also make sure to invoke the constant to ensure it gets compiled.
    quote! {
        #flagged

        trait Verifier {
            const VALID: ();
        }

        impl #impl_generics Verifier for #derived #ty_generics {
            const VALID: () = assert!(
                Checker::<(#left, #derived #ty_generics)>::FLAG ^ Checker::<(#right, #derived #ty_generics)>::FLAG,
                "first and last types are equal"
            );
        }

        let _ = #derived #turbofish::VALID;
    }
}
