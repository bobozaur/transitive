#![allow(unused_variables)]
#![allow(unused_must_use)]

mod common;

use transitive::TransitiveTryFrom;

#[derive(TransitiveTryFrom)]
#[transitive(all(D, C, B))] // impl TryFrom<D> and TryFrom<C> for A
struct A;
struct B;
struct C;
struct D;

impl_try_from!(B to A);
impl_try_from!(C to B);
impl_try_from!(D to C);

#[test]
fn try_from() {
    A::try_from(D);

    // Compiles:
    A::try_from(C);
}
