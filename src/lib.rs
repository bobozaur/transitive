//! This crate provides derive macros that take care of boilerplate code to make
//! transitive conversions in Rust using [`From`] and [`TryFrom`] traits.
//!
//! It's not magic and it is completely static. The derives here merely implement [`From`] or [`TryFrom`]
//! between types by relying on already defined impls between types provided in the path.
//!
//! The path taken for transitions must be annotated (correctly) for transitions to work.
//! Additonally, there can only be one transition between a source and a target type,
//! as otherwise there would be duplicate trait implementations.
//!
//! The path is provided in the [`#[transitive]`] attribute along with a direction:
//! - `#[transitive(from(A, B, C))]` results in A -> B -> C -> derived type
//! - `#[transitive(into(A, B, C))]` results in derived type -> A -> B -> C
//! - `#[transitive(from(all(A, B, C)))]` treats every element apart from the last as a source type
//! - `#[transitive(into(all(A, B, C)))]` treats every element apart from the first as a target type
//!
//! For [`TransitiveTryFrom`] the error types must be convertible to the error type of the last conversion taking place.
//! Additionally, the attribute arguments become `try_from` and `try_into`.
//!
//! # Conversions table:
//!
//! | Derived Type | Derive macro             | Annotation                        | Will impl           | Conditions                                                                                                                |
//! |--------------|--------------------------|-----------------------------------|---------------------|---------------------------------------------------------------------------------------------------------------------------|
//! | A            | [`TransitiveFrom`]       | #[transitive(into(B, C, D))]      | `From<A> for D`     | `From<A> for B`; `From<B> for C`; `From<C> for D`                                                                         |
//! | A            | [`TransitiveFrom`]       | #[transitive(from(D, C, B))]      | `From<D> for A`     | `From<D> for C`; `From<C> for B`; `From<B> for A`                                                                         |
//! | A            | [`TransitiveTryFrom`]    | #[transitive(try_into(B, C, D))]  | `TryFrom<A> for D`  | `TryFrom<A> for B`; `TryFrom<B> for C`; `TryFrom<C> for D`; errors must impl `From<ErrType> for <D as TryFrom<C>>::Error` |
//! | A            | [`TransitiveTryFrom`]    | #[transitive(try_from(D, C, B))]  | `TryFrom<D> for A`  | `TryFrom<D> for C`; `TryFrom<C>` for B; `TryFrom<B> for A`; errors must `impl From<ErrType> for <A as TryFrom<B>>::Error` |
//!
//!
//! # Examples:
//!
//! Assume you have types `A`, `B`, `C` and `D`:
//!
//! ```
//! use transitive::TransitiveFrom;
//!
//! #[derive(TransitiveFrom)]
//! #[transitive(into(B, C, D))] // impl From<A> for D by doing A -> B -> C -> D
//! struct A;
//!
//! #[derive(TransitiveFrom)]
//! #[transitive(into(C, D))] // impl From<B> for  D by doing B -> C -> D
//! struct B;
//! struct C;
//! struct D;
//!
//! impl From<A> for B {
//!     fn from(val: A) -> Self {
//!         Self
//!     }
//! };
//!
//! impl From<B> for C {
//!     fn from(val: B) -> Self {
//!         Self
//!     }
//! };
//!
//! impl From<C> for D {
//!     fn from(val: C) -> Self {
//!         Self
//!     }
//! };
//!
//! #[test]
//! fn into() {
//!     D::from(A);
//!     D::from(B);
//! }
//! ```
//!
//! The derives support multiple `transitive` attribute instances, each providing a list of types as a path:
//!
//! ```
//! use transitive::TransitiveFrom;
//!
//! #[derive(TransitiveFrom)]
//! #[transitive(into(B, C))] // impl From<A> for C by doing A -> B -> C
//! #[transitive(into(C, D))] // impl From<A> for D by doing A -> C -> D
//! struct A;
//! struct B;
//! struct C;
//! struct D;
//!
//! impl From<A> for B {
//!     fn from(val: A) -> Self {
//!         Self
//!     }
//! };
//!
//! impl From<B> for C {
//!     fn from(val: B) -> Self {
//!         Self
//!     }
//! };
//!
//! impl From<C> for D {
//!     fn from(val: C) -> Self {
//!         Self
//!     }
//! };
//!
//! #[test]
//! fn into() {
//!     D::from(A);
//! }
//! ```
//!
//! To avoid a lot of attribte annotations, you can encapsulate the type list in `all()`
//! to which will cause every type after the first to be considered a target type, hence
//! generating an impl:
//!
//! ```
//! use transitive::TransitiveFrom;
//!
//! #[derive(TransitiveFrom)]
//! #[transitive(into(all(B, C, D, E, F)))] // impl From<A> for C, D, E and F
//! struct A;
//! struct B;
//! struct C;
//! struct D;
//! struct E;
//! struct F;
//!
//! impl From<A> for B {
//!     fn from(val: A) -> Self {
//!         Self
//!     }
//! };
//!
//! impl From<B> for C {
//!     fn from(val: B) -> Self {
//!         Self
//!     }
//! };
//!
//! impl From<C> for D {
//!     fn from(val: C) -> Self {
//!         Self
//!     }
//! };
//!
//! impl From<D> for E {
//!     fn from(val: D) -> Self {
//!         Self
//!     }
//! };
//!
//! impl From<E> for F {
//!     fn from(val: E) -> Self {
//!         Self
//!     }
//! };
//!
//! #[test]
//! fn into() {
//!     C::from(A);
//!     D::from(A);
//!     E::from(A);
//!     F::from(A);
//! }
//! ```
//!
//! Let's see an example on how to use [`TransitiveTryFrom`] which combines the "reversed"
//! nature of the `from` attribute modifier and the error transitions constraints:
//!
//! ```
//! use transitive::{TransitiveTryFrom, TransitiveFrom};
//!
//! // Note how the annotation now considers `A` as
//! // target type and `D`, the first element in the type list,
//! // as source.
//! #[derive(TransitiveTryFrom)]
//! #[transitive(try_from(D, C, B))] // impl TryFrom<D> for A
//! struct A;
//!
//! #[derive(TransitiveTryFrom)]
//! #[transitive(try_from(D, C))] // impl TryFrom<D> for B
//! struct B;
//! struct C;
//! struct D;
//!
//! struct ErrD_C;
//! struct ErrC_B;
//!
//! #[derive(TransitiveFrom)]
//! #[transitive(from(ErrD_C, ErrC_B))] // impl From<ErrD_C> for ErrB_A
//! struct ErrB_A;
//!
//! impl From<ErrD_C> for ErrC_B {
//!     fn from(val: ErrD_C) -> Self {
//!         Self
//!     }
//! };
//!
//! impl From<ErrC_B> for ErrB_A {
//!     fn from(val: ErrC_B) -> Self {
//!         Self
//!     }
//! };
//!
//! impl TryFrom<D> for C {
//!     type Error = ErrD_C;
//!
//!     fn try_from(val: D) -> Result<Self, Self::Error> {
//!         Ok(Self)    
//!     }
//! };
//!
//! impl TryFrom<C> for B {
//!     type Error = ErrC_B;
//!
//!     fn try_from(val: C) -> Result<Self, Self::Error> {
//!         Ok(Self)    
//!     }
//! };
//!
//! impl TryFrom<B> for A {
//!     type Error = ErrB_A;
//!
//!     fn try_from(val: B) -> Result<Self, Self::Error> {
//!         Ok(Self)    
//!     }
//! };
//!
//! #[test]
//! fn try_from() {
//!     A::try_from(D);
//!     B::try_from(D);
//! }
//! ```

#![allow(clippy::expect_fun_call)]

mod transitive;

use crate::transitive::{
    direction_handler::DirectionHandler, fallible::FallibleTransition,
    infallible::InfallibleTransition,
};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

/// Derive macro that implements [From] for infallible transitions.
#[proc_macro_derive(TransitiveFrom, attributes(transitive))]
pub fn transitive_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    InfallibleTransition
        .generate_tokens(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive macro that implements [TryFrom] for fallible transitions.
/// The error types occurring through the transition must all be convertible
/// (implement [`From`]) to the error type of the **last** transition in the path.
#[proc_macro_derive(TransitiveTryFrom, attributes(transitive))]
pub fn transitive_try_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    FallibleTransition
        .generate_tokens(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
