#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_camel_case_types)]

mod common;

use transitive::Transitive;

#[derive(Transitive)]
#[transitive(try_into(B, C, D), error = "ConvErr")] // impl TryFrom<A> for D
struct A;
struct B;
struct C;
struct D;

struct ConvErr;
struct ErrA_B;
struct ErrB_C;
struct ErrC_D;

impl From<ErrA_B> for ConvErr {
    fn from(value: ErrA_B) -> Self {
        Self
    }
}

impl From<ErrB_C> for ConvErr {
    fn from(value: ErrB_C) -> Self {
        Self
    }
}

impl From<ErrC_D> for ConvErr {
    fn from(value: ErrC_D) -> Self {
        Self
    }
}

impl_try_from!(A to B err ErrA_B);
impl_try_from!(B to C err ErrB_C);
impl_try_from!(C to D err ErrC_D);

#[test]
fn try_into() {
    D::try_from(A);
}
