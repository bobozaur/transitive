use std::marker::PhantomData;

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

#[derive(Transitive)]
#[transitive(from(D, C, B, A))] // impl From<D> for Z<T>
struct Z<T>(PhantomData<T>);

impl<T> From<A> for Z<T> {
    fn from(value: A) -> Self {
        Self(PhantomData)
    }
}

pub fn from() {
    A::from(D);
    B::from(D);
}
