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
//! Note that [`TransitiveInto`] and [`TransitiveTryInto`], despite the name, 
//! implement [`From`] or [`TryFrom`] from the derived type to the last type in the path.
//! 
//! Also, [`TransitiveFrom`] and [`TransitiveTryFrom`] are mirror types that
//! convert from the first type in the path, transitioning through successive types, to the
//! derived type. This also happens by implementing [`From`] and [`TryFrom`].
//! 
//! For [`TransitiveTryInto`] and [`TransitiveTryFrom`] the error types must be convertible
//! to the error type of the last conversion taking place.
//! 
//! # Conversions table:
//! | Derived Type | Derive macro            | Annotation              | Will impl           | Conditions                                                                                                                |
//! |--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | A            | [`TransientInto`]       | #[transitive(B, C, D)]  | `From<A> for D`     | `From<A> for B`, `From<B> for C`, `From<C> for D`                                                                         |
//! | A            | [`TransientFrom`]       | #[transitive(D, C, B)]  | `From<D> for A`     | `From<D> for C`, `From<C> for B`, `From<B> for A`                                                                         |
//! | A            | [`TransientTryInto`]    | #[transitive(B, C, D)]  | `TryFrom<A> for D`  | `TryFrom<A> for B`, `TryFrom<B> for C`, `TryFrom<C> for D`, errors must impl `From<ErrType> for <D as TryFrom<C>>::Error` |
//! | A            | [`TransientTryFrom`]    | #[transitive(D, C, B)]  | `TryFrom<D> for A`  | `TryFrom<D> for C`, `TryFrom<C>` for B, `TryFrom<B> for A`, errors must `impl From<ErrType> for <A as TryFrom<B>>::Error` |
//! 
//! # Examples:
//! 
//! Assume you have types `A`, `B`, `C` and `D`:
//! 
//! ``` ignore
//! use transitive::TransitiveInto;
//!
//! #[derive(TransitiveInto)]
//! #[transitive(B, C, D)] // impl From<A> for D by doing A -> B -> C -> D
//! struct A;
//!
//! #[derive(TransitiveInto)]
//! #[transitive(C, D)] // impl From<B> for  D by doing B -> C -> D
//! struct B;
//! struct C;
//! struct D;
//!
//! impl From<A> for B {...};
//! impl From<B> for C {...};
//! impl From<C> for D {...};
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
//! ``` ignore
//! use transitive::TransitiveInto;
//!
//! #[derive(TransitiveInto)]
//! #[transitive(B, C)] // impl From<A> for C by doing A -> B -> C
//! #[transitive(C, D)] // impl From<A> for D by doing A -> C -> D
//! struct A;
//! struct B;
//! struct C;
//! struct D;
//!
//! impl From<A> for B {...};
//! impl From<B> for C {...};
//! impl From<C> for D {...};
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
//! ``` ignore
//! use transitive::TransitiveInto;
//!
//! #[derive(TransitiveInto)]
//! #[transitive(all(B, C, D, E, F))] // impl From<A> for C, D, E and F
//! struct A;
//! struct B;
//! struct C;
//! struct D;
//! struct E;
//! struct F;
//!
//! impl From<A> for B {...};
//! impl From<B> for C {...};
//! impl From<C> for D {...};
//! impl From<D> for E {...};
//! impl From<E> for F {...};
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
//! Let's see an example on how to use [`TransitiveTryFrom`] which combines the "reversed" nature
//! of the type list also used in [`TransitiveFrom`] and the error transitions also used in [`TransitiveTryInto`]:
//! 
//! ```
//! use transitive::{TransitiveTryFrom, TransitiveFrom};
//!
//! // Note how the annotation now considers `A` as
//! // target type and `D`, the first element in the type list,
//! // as source.
//! #[derive(TransitiveTryFrom)]
//! #[transitive(D, C, B)] // impl TryFrom<D> for A
//! struct A;
//!
//! #[derive(TransitiveTryFrom)]
//! #[transitive(D, C)] // impl TryFrom<D> for B
//! struct B;
//! struct C;
//! struct D;
//! 
//! struct ErrD_C;
//! struct ErrC_B;
//! #[derive(TransitiveFrom)]
//! #[transitive(ErrD_C, ErrC_B)] // impl From<ErrD_C> for ErrB_A
//! struct ErrB_A;
//! 
//! impl From<ErrD_C> for ErrC_B {}
//! impl From<ErrC_B> for ErrB_A {}
//!
//! impl TryFrom<D> for C {type Error = ErrD_C; ...};
//! impl TryFrom<C> for B {type Error = ErrC_B; ...};
//! impl TryFrom<B> for A {type Error = ErrB_A; ...};
//!
//! #[test]
//! fn try_from() {
//!     A::try_from(D);
//!     B::try_from(D);
//! }
//! ```

#![allow(clippy::expect_fun_call)]

mod transitive;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error};
use transitive::{from, into, transitive_impl, try_from, try_into};

/// Derive macro that implements [From] for A -> C by converting A -> B -> C,
/// where A is the derived type and C is the **last** type in the transition chain.
/// 
/// For this to work, [`From`] A to B and [`From`] B to C impls must exist.
/// The `transitive` attribute is where the list of types to transit through is provided, in order.

#[proc_macro_derive(TransitiveInto, attributes(transitive))]
pub fn transitive_into(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let conv_func = quote! {from};
    transitive_impl(
        input,
        &conv_func,
        &into::ts_maker,
        false,
        &into::create_from_impl,
    )
    .unwrap_or_else(Error::into_compile_error)
    .into()
}

/// Derive macro that implements [From] for C -> A by converting C -> B -> A,
/// where A is the derived type and C is the **first** type in the transition chain.
/// 
/// For this to work, [`From`] C to B and [`From`] B to A impls must exist.
/// The `transitive` attribute is where the list of types to transit through is provided, in order.
#[proc_macro_derive(TransitiveFrom, attributes(transitive))]
pub fn transitive_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let conv_func = quote! {into};
    transitive_impl(
        input,
        &conv_func,
        &from::ts_maker,
        false,
        &from::create_from_impl,
    )
    .unwrap_or_else(Error::into_compile_error)
    .into()
}


/// Derive macro that implements [TryFrom] for A -> C by converting A -> B -> C,
/// where A is the derived type and C is the **last** type in the transition chain.
/// 
/// For this to work, [`TryFrom`] A to B and [`TryFrom`] B to C impls must exist AND
/// the error types that can be returned by the intermediary [`TryFrom`] impls must be
/// convertible to `<C as TryFrom<B>>::Error`.
/// 
/// The `transitive` attribute is where the list of types to transit through is provided, in order.
#[proc_macro_derive(TransitiveTryInto, attributes(transitive))]
pub fn transitive_try_into(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let conv_func = quote! {try_from};
    transitive_impl(
        input,
        &conv_func,
        &try_into::ts_maker,
        true,
        &try_into::create_try_from_impl,
    )
    .unwrap_or_else(Error::into_compile_error)
    .into()
}

/// Derive macro that implements [TryFrom] for C -> A by converting C -> B -> A,
/// where A is the derived type and C is the **first** type in the transition chain.
/// 
/// For this to work, [`TryFrom`] C to B and [`TryFrom`] B to A impls must exist AND
/// the error types that can be returned by the intermediary [`TryFrom`] impls must be
/// convertible to `<A as TryFrom<B>>::Error`.
/// 
/// The `transitive` attribute is where the list of types to transit through is provided, in order.
#[proc_macro_derive(TransitiveTryFrom, attributes(transitive))]
pub fn transitive_try_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let conv_func = quote! {try_into};
    transitive_impl(
        input,
        &conv_func,
        &try_from::ts_maker,
        true,
        &try_from::create_try_from_impl,
    )
    .unwrap_or_else(Error::into_compile_error)
    .into()
}
