use transitive::Transitive;

use crate::impl_from;

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

pub fn from() {
    A::from(D);
    B::from(D);
}
