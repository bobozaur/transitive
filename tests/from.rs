mod macros;

use std::marker::PhantomData;

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

#[derive(Transitive)]
#[transitive(from(D, C, B, A))] // impl From<D> for Z<T>
struct Z<T>(PhantomData<T>);

impl<T> From<A> for Z<T> {
    fn from(_value: A) -> Self {
        Self(PhantomData)
    }
}

#[test]
pub fn test_from() {
    let _ = A::from(D);
    let _ = B::from(D);
}
