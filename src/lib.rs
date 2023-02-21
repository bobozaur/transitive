//! This crate provides derive macros that take care of boilerplate code to make
//! transitive conversions in Rust using [`From`] and [`TryFrom`] traits.

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
