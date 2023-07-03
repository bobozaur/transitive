#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_camel_case_types)]

mod common;

use transitive::Transitive;

#[derive(Transitive)]
#[transitive(try_from(D, C, B), error = "ConvErr")] // impl TryFrom<D> for A
struct A;
struct B;
struct C;
struct D;

struct ConvErr;

struct ErrD_C;
struct ErrC_B;
struct ErrB_A;

impl From<ErrD_C> for ConvErr {
    fn from(value: ErrD_C) -> Self {
        Self
    }
}

impl From<ErrC_B> for ConvErr {
    fn from(value: ErrC_B) -> Self {
        Self
    }
}

impl From<ErrB_A> for ConvErr {
    fn from(value: ErrB_A) -> Self {
        Self
    }
}

impl_try_from!(B to A err ErrB_A);
impl_try_from!(C to B err ErrC_B);
impl_try_from!(D to C err ErrD_C);

#[test]
fn try_from() {
    A::try_from(D);
}
