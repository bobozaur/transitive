#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_camel_case_types)]

mod common;

use transitive::{TransitiveFrom, TransitiveTryInto};

#[derive(TransitiveTryInto)]
#[transitive(B, C, D)] // impl TryFrom<A> for D
struct A;

#[derive(TransitiveTryInto)]
#[transitive(C, D)] // impl TryFrom<B> for  D
struct B;
struct C;
struct D;

struct ErrA_B;
struct ErrB_C;
#[derive(TransitiveFrom)]
#[transitive(ErrA_B, ErrB_C)] // impl From<ErrA_B> for ErrC_D
struct ErrC_D;

impl From<ErrA_B> for ErrB_C {
    fn from(value: ErrA_B) -> Self {
        Self
    }
}

impl From<ErrB_C> for ErrC_D {
    fn from(value: ErrB_C) -> Self {
        Self
    }
}

impl_try_from!(A to B err ErrA_B);
impl_try_from!(B to C err ErrB_C);
impl_try_from!(C to D err ErrC_D);

#[test]
fn try_into() {
    D::try_from(A);
    D::try_from(B);

    // should not compile:
    // C::try_from(A);
}
