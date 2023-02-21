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
//! | Derived Type | Derive macro | Annotation              | Will impl           | Conditions                                                                                                              |
//! |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | A | [`TransientInto`]       | #[transitive(B, C, D)]  | `From<A> for D`     | `From<A> for B`, `From<B> for C`, `From<C> for D`                                                                       |
//! | A | [`TransientFrom`]       | #[transitive(D, C, B)]  | `From<D> for A`     | `From<D> for C`, `From<C> for B`, `From<B> for A`                                                                       |
//! | A | [`TransientTryInto`]    | #[transitive(B, C, D)]  | `TryFrom<A> for D`  | `TryFrom<A> for B`, `TryFrom<B> for C`, `TryFrom<C> for D`, errors must impl From<ErrType> for <D as TryFrom<C>>::Error |
//! | A | [`TransientTryFrom`]    | #[transitive(D, C, B)]  | `TryFrom<D> for A`  | `TryFrom<D> for C`, `TryFrom<C>` for B, `TryFrom<B> for A, errors must impl From<ErrType> for <A as TryFrom<B>>::Error` |
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
//! to allow transitions between all the intermediary "pitstops":
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
//! Let's see an example on how to use [`TransitiveTryFrom`]:
//! 

#![allow(clippy::expect_fun_call)]

mod transitive;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error};
use transitive::{from, into, transitive_impl, try_from, try_into};

/// Derive macro that implements [From] for A -> C by converting A -> B -> C.
/// For this to work, [`From`] A to B and [`From`] B to C impls must exist.
/// The attribute is where the list of types to transit through is provided, in order.
///
/// Multiple attributes can be used on a single type for multiple transitive [`From`] impls
/// and the trasitions chain is virtually unlimited.
///
/// The macro supports two attributes, with slightly different behavior:
/// * `transitive` -> For A, B, C, D it derives `impl From<A> for D`, skipping `impl From<A> for C`
/// * `transitive_all` -> For A, B, C, D it derives `impl From<A> for D` *AND* `impl From<A> for C`

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

/// **NOTE**: This macro uses the argument types provided in the attributes
/// in **reverse order**.
///
///  Derive macro that implements [TryFrom] for C -> A by converting C -> B -> A.
///
/// This works similarly as the [`TransitiveInto`] derive macro, but for the purpose
/// of reusing the same attributes and type lists, it parses them *in reverse*.
///
/// So if you can do From: A -> B -> C you can do TryFrom: C -> B -> A.
///
/// Just like with [`TransitiveInto`], [`TryFrom`] B to A and [`TryFrom`] C to B impls must exist.
/// The attribute is where the list of types to transit through is provided, in *reverse* order.
///
/// Multiple attributes can be used on a single type for multiple transitive [`TryFrom`] impls
/// and the trasitions chain is virtually unlimited.
///
/// The macro supports two attributes, with slightly different behavior:
/// * `transitive` -> For A, B, C, D it derives `impl TryFrom<D> for A`, skipping `impl TryFrom<C> for A`
/// * `transitive_all` -> For A, B, C, D it derives `impl TryFrom<D> for A` *AND* `impl TryFrom<C> for A`
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
