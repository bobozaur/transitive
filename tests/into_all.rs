#![allow(unused_variables)]
#![allow(unused_must_use)]

mod common;

use transitive::TransitiveInto;

#[derive(TransitiveInto)]
#[transitive(all(B, C, D))] // impl From<A> for C and D
struct A;
struct B;
struct C;
struct D;

impl_from!(A to B);
impl_from!(B to C);
impl_from!(C to D);

#[test]
fn into_all() {
    D::from(A);

    // Compiles:
    C::from(A);
}
