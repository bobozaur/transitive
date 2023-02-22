#![allow(unused_variables)]
#![allow(unused_must_use)]

mod common;

use transitive::{TransitiveFrom, TransitiveTryFrom};

#[derive(TransitiveFrom, TransitiveTryFrom)]
#[transitive(from(all(D, C, B)))] // impl From<D> and From<C> for A
#[transitive(try_into(B, C, D))] // impl TryFrom<A> for D
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
    D::try_from(A);
}
