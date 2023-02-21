#![allow(unused_variables)]
#![allow(unused_must_use)]

mod common;

use transitive::TransitiveFrom;

#[derive(TransitiveFrom)]
#[transitive(all(D, C, B))] // impl From<D> and From<C> for A
struct A;
struct B;
struct C;
struct D;

impl_from!(B to A);
impl_from!(C to B);
impl_from!(D to C);

#[test]
fn from_all() {
    A::from(D);

    // Compiles:
    A::from(C);
}
