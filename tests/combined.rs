#![allow(unused_variables)]
#![allow(unused_must_use)]

mod common;

use transitive::TransitiveFrom;

#[derive(TransitiveFrom)]
#[transitive(from(all(D, C, B)), into(B, C, D))] // impl From<D> and From<C> for A
struct A;
struct B;
struct C;
struct D;

impl_from!(B to A);
impl_from!(C to B);
impl_from!(D to C);

impl_from!(A to B);
impl_from!(B to C);
impl_from!(C to D);

#[test]
fn from_all() {
    A::from(D);
    A::from(C);
    D::from(A);
}
