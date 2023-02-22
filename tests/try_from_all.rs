#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_camel_case_types)]

mod common;

use transitive::{TransitiveFrom, TransitiveTryFrom};

#[derive(TransitiveTryFrom)]
#[transitive(try_from(all(D, C, B)))] // impl TryFrom<D> and TryFrom<C> for A
struct A;
struct B;
struct C;
struct D;

struct ErrD_C;
struct ErrC_B;
#[derive(TransitiveFrom)]
#[transitive(from(ErrD_C, ErrC_B))] // impl From<ErrD_C> for ErrB_A
struct ErrB_A;

impl From<ErrD_C> for ErrC_B {
    fn from(value: ErrD_C) -> Self {
        Self
    }
}

impl From<ErrC_B> for ErrB_A {
    fn from(value: ErrC_B) -> Self {
        Self
    }
}

impl_try_from!(B to A err ErrB_A);
impl_try_from!(C to B err ErrC_B);
impl_try_from!(D to C err ErrD_C);

#[test]
fn try_from_all() {
    A::try_from(D);

    // Compiles:
    A::try_from(C);
}
