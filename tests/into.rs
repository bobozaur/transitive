#![allow(unused_variables)]
#![allow(unused_must_use)]

mod common;

use transitive::TransitiveInto;

#[derive(TransitiveInto)]
#[transitive(B, C, D)] // impl From<A> for D
struct A;

#[derive(TransitiveInto)]
#[transitive(C, D)] // impl From<B> for  D
struct B;
struct C;
struct D;

impl_from!(A to B);
impl_from!(B to C);
impl_from!(C to D);

#[test]
fn into() {
    D::from(A);
    D::from(B);

    // should not compile:
    // C::from(A);
}