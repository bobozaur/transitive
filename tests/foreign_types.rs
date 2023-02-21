#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_camel_case_types)]

mod common;

use std::num::ParseIntError;

use transitive::{TransitiveTryFrom, TransitiveFrom};

#[derive(TransitiveTryFrom)]
#[transitive(u8, C, B)] // impl TryFrom<u8> for A
struct A;

#[derive(TransitiveTryFrom)]
#[transitive(u8, C)] // impl TryFrom<D> for B
struct B;
struct C;

struct ErrC_B;
#[derive(TransitiveFrom)]
#[transitive(ParseIntError, ErrC_B)] // impl From<Err_u8_C> for ErrB_A
struct ErrB_A;

impl From<ParseIntError> for ErrC_B {
    fn from(value: ParseIntError) -> Self {
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
impl_try_from!(u8 to C err ParseIntError);

#[test]
fn foreign_types() {
    A::try_from(1);
    B::try_from(1);

    // should not compile:
    // A::try_from(C);
}
