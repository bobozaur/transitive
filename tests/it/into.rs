use transitive::Transitive;

use crate::impl_from;

#[derive(Transitive)]
#[transitive(into(B, C, D))] // impl From<A> for D
struct A;

#[derive(Transitive)]
#[transitive(into(C, D))] // impl From<B> for  D
struct B;
struct C;
struct D;

impl_from!(A to B);
impl_from!(B to C);
impl_from!(C to D);

pub fn into() {
    D::from(A);
    D::from(B);
}
