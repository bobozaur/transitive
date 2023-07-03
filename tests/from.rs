#![allow(unused_variables)]
#![allow(unused_must_use)]

mod common;

use transitive::Transitive;

#[derive(Transitive)]
#[transitive(from(D, C, B))] // impl From<D> for A
struct A;

#[derive(Transitive)]
#[transitive(from(D, C))] // impl From<D> for A
struct B;
struct C;
struct D;

impl_from!(B to A);
impl_from!(C to B);
impl_from!(D to C);

#[test]
fn from() {
    A::from(D);
    B::from(D);
}
