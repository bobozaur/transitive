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
//! The path is provided in the [`#[transitive]`] attribute along with a direction. Assuming we derive this on type `X`:
//! - `#[transitive(from(A, B, C))]` results in: A -> B -> C -> X
//! - `#[transitive(into(A, B, C))]` results in: X -> A -> B -> C
//! - `#[transitive(try_from(A, B, C))]` results in the fallible version of: A -> B -> C -> X
//! - `#[transitive(try_into(A, B, C))]` results in the fallible version of: X -> A -> B -> C
//!
//! By default, for fallible conversions the error types must be convertible to the error type of the last conversion taking place.
//! So, in a `#[transitive(try_from(A, B, C))]` annotation on type `X`, the last conversion taking place is `C -> X`.
//! Let's call this `ErrC_X`. All error types resulting from the previous conversions must implement `From<Err> for ErrC_X`.
//!
//! # Conversions table:
//!
//! | Derived Type | Annotation                        | Will impl           | Conditions                                                                                                                |
//! |--------------|-----------------------------------|---------------------|---------------------------------------------------------------------------------------------------------------------------|
//! | A            | #[transitive(into(B, C, D))]      | `From<A> for D`     | `From<A> for B`; `From<B> for C`; `From<C> for D`                                                                         |
//! | A            | #[transitive(from(D, C, B))]      | `From<D> for A`     | `From<D> for C`; `From<C> for B`; `From<B> for A`                                                                         |
//! | A            | #[transitive(try_into(B, C, D))]  | `TryFrom<A> for D`  | `TryFrom<A> for B`; `TryFrom<B> for C`; `TryFrom<C> for D`; errors must impl `From<ErrType> for <D as TryFrom<C>>::Error` |
//! | A            | #[transitive(try_from(D, C, B))]  | `TryFrom<D> for A`  | `TryFrom<D> for C`; `TryFrom<C>` for B; `TryFrom<B> for A`; errors must `impl From<ErrType> for <A as TryFrom<B>>::Error` |
//!
//!
//! # Custom error type:
//!
//! For `try_from` and `try_into` annotations, the macro attribute can accept an `error = "MyError"` argument,
//! like so: `#[transitive(try_into(A, B, C), error = "MyError")]`. This overrides the default behavior and allows
//! specifying a custom error type, but all the error types resulting from conversions must be convertible to this type.
//!
//! # Examples:
//!
//! Assume you have types `A`, `B`, `C` and `D`:
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
//! The derive supports multiple `transitive` attribute instances, each providing a list of types as a path:
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
//! use transitive::{Transitive};
//!
//! // Note how the annotation now considers `A` as target type
//! // and `D`, the first element in the type list, as source.
//! #[derive(Transitive)]
//! #[transitive(try_from(D, C, B))] // impl TryFrom<D> for A
//! struct A;
//!
//! #[derive(Transitive)]
//! #[transitive(try_from(D, C), error = "ConvErr")] // impl TryFrom<D> for B, with custom error
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
use syn::{parse_macro_input, DeriveInput, Error};

/// Derive macro that implements [From] for infallible transitions.
#[proc_macro_derive(Transitive, attributes(transitive))]
pub fn transitive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    transitive::transitive_impl(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
