#![allow(unused_variables)]
#![allow(unused_must_use)]

mod common;

use transitive::TransitiveTryFrom;

#[derive(TransitiveTryFrom)]
#[transitive(D, C, B)] // impl TryFrom<D> for A
struct A;

#[derive(TransitiveTryFrom)]
#[transitive(D, C)] // impl TryFrom<D> for B
struct B;
struct C;
struct D;

impl_try_from!(B to A);
impl_try_from!(C to B);
impl_try_from!(D to C);

#[test]
fn try_from() {
    A::try_from(D);
    B::try_from(D);

    // should not compile:
    // A::try_from(C);
}
