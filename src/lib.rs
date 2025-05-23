//! This crate provides a derive macro that takes care of boilerplate code to make transitive
//! conversions in Rust using [`From`] and [`TryFrom`] traits.
//!
//! It's not magic and it's completely static. The derive here merely implements [`From`] or
//! [`TryFrom`] between types by relying on existent impls between items in the path.
//!
//! The path taken for transitions must be annotated (correctly) for transitions to work.
//! Additonally, there can only be one transition between a source and a target type, as otherwise
//! there would be duplicate trait implementations.
//!
//! The path is provided in the [`#[transitive]`] attribute along with a direction:
//!
//! ```ignore, compile_fail
//! #[derive(Transitive)]
//! #[transitive(from(D, C, B))] // Results in `impl From<D> for A` as `D -> C -> B -> A`
//! #[transitive(into(B, C, D))] // Results in `impl From<A> for D` as `A -> B -> C -> D`
//! struct A;
//! ```
//!
//! # Conversions table:
//!
//! | Derived Type | Annotation                        | Will impl           | Conditions                                                                                                                |
//! |--------------|-----------------------------------|---------------------|---------------------------------------------------------------------------------------------------------------------------|
//! | A            | #[transitive(into(B, C, D))]      | `From<A> for D`     | `From<A> for B`; `From<B> for C`; `From<C> for D`                                                                         |
//! | A            | #[transitive(from(D, C, B))]      | `From<D> for A`     | `From<D> for C`; `From<C> for B`; `From<B> for A`                                                                         |
//! | A            | #[transitive(try_into(B, C, D))]  | `TryFrom<A> for D`  | `TryFrom<A> for B`; `TryFrom<B> for C`; `TryFrom<C> for D`; errors must impl `From<ErrType> for <D as TryFrom<C>>::Error` |
//! | A            | #[transitive(try_from(D, C, B))]  | `TryFrom<D> for A`  | `TryFrom<D> for C`; `TryFrom<C> for B`; `TryFrom<B> for A`; errors must impl `From<ErrType> for <A as TryFrom<B>>::Error` |
//!
//!
//! # Custom error type:
//!
//! For `try_from` and `try_into` annotations, the macro attribute can accept an `error = MyError`
//! argument as the last element, like so: `#[transitive(try_into(A, B, C, error = MyError))]`. This
//! overrides the default behavior and allows specifying a custom error type, but all the error
//! types resulting from conversions must be convertible to this type.
//!
//! # Examples:
//!
//! ```
//! use transitive::Transitive;
//!
//! #[derive(Transitive)]
//! #[transitive(into(B, C, D))] // impl From<A> for D by doing A -> B -> C -> D
//! struct A;
//!
//! #[derive(Transitive)]
//! #[transitive(into(C, D))] // impl From<B> for D by doing B -> C -> D
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
//! D::from(A);
//! D::from(B);
//! ```
//!
//! Note that the macro does nothing for types in the middle:
//!
//! ```compile_fail
//! use transitive::Transitive;
//!
//! #[derive(Transitive)]
//! #[transitive(into(B, C, D))] // impl From<A> for D by doing A -> B -> C -> D
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
//! D::from(A); // works
//! C::from(A); // does not compile
//! ```
//!
//! ```
//! use transitive::Transitive;
//!
//! #[derive(Transitive)]
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
//! D::from(A);
//! ```
//!
//! Let's see an example on how to use [`Transitive`] when combining the "reversed"
//! nature of the `from` and `try_from` attribute modifiers and the error transitions constraints:
//!
//! ```
//! #![allow(non_camel_case_types)]
//! use transitive::Transitive;
//!
//! // Note how the annotation now considers `A` as target type
//! // and `D`, the first element in the type list, as source.
//! #[derive(Transitive)]
//! #[transitive(try_from(D, C, B))] // impl TryFrom<D> for A
//! struct A;
//!
//! #[derive(Transitive)]
//! #[transitive(try_from(D, C, error = ConvErr))] // impl TryFrom<D> for B, with custom error
//! struct B;
//! struct C;
//! struct D;
//!
//! struct ConvErr;
//!
//! struct ErrD_C;
//! struct ErrC_B;
//!
//! #[derive(Transitive)]
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
//! impl From<ErrD_C> for ConvErr {
//!     fn from(val: ErrD_C) -> Self {
//!         Self
//!     }
//! };
//!
//! impl From<ErrC_B> for ConvErr {
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
//! A::try_from(D);
//! B::try_from(D);
//! ```

mod transitive;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

use crate::transitive::TransitiveInput;

#[proc_macro_derive(Transitive, attributes(transitive))]
pub fn transitive(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as TransitiveInput)
        .to_token_stream()
        .into()
}
