#![allow(unused_variables)]
#![allow(unused_must_use)]

mod common;

use transitive::TransitiveTryInto;

#[derive(TransitiveTryInto)]
#[transitive(B, C, D)] // impl TryFrom<A> for D
struct A;

#[derive(TransitiveTryInto)]
#[transitive(C, D)] // impl TryFrom<B> for  D
struct B;
struct C;
struct D;

impl_try_from!(A to B);
impl_try_from!(B to C);
impl_try_from!(C to D);

#[test]
fn try_into() {
    D::try_from(A);
    D::try_from(B);

    // should not compile:
    // C::try_from(A);
}
