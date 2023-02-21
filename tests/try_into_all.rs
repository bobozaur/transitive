#![allow(unused_variables)]
#![allow(unused_must_use)]

mod common;

use transitive::TransitiveTryInto;

#[derive(TransitiveTryInto)]
#[transitive(all(B, C, D))] // impl TryFrom<A> for C and D
struct A;
struct B;
struct C;
struct D;

impl_try_from!(A to B);
impl_try_from!(B to C);
impl_try_from!(C to D);

#[test]
fn try_into_all() {
    D::try_from(A);

    // Compiles:
    C::try_from(A);
}
