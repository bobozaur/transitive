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

#[allow(clippy::duplicated_attributes)]
#[derive(Transitive)]
#[transitive(from(D, C, B, A))] // impl From<D> for Z<T>
#[transitive(from(C, B))] // impl From<D> for Z<T>
struct Z<T>(PhantomData<T>);

impl<T> From<A> for Z<T> {
    fn from(_value: A) -> Self {
        Self(PhantomData)
    }
}

impl<T> From<B> for Z<T> {
    fn from(_value: B) -> Self {
        Self(PhantomData)
    }
}

#[derive(Transitive)]
#[transitive(from(D, C, B, A))] // impl From<D> for Y<'a>
struct Y<'a>(PhantomData<&'a ()>);

impl From<A> for Y<'_> {
    fn from(_value: A) -> Self {
        Self(PhantomData)
    }
}

#[derive(Transitive)]
#[transitive(from(D, C, B, A))] // impl From<D> for W<N>
struct W<const N: usize>;

impl<const N: usize> From<A> for W<N> {
    fn from(_value: A) -> Self {
        Self
    }
}

#[derive(Transitive)]
#[transitive(from(D, C, B, A))] // impl From<D> for Q<'a, 'b, N, T, U>
struct Q<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b = &'b str>(
    PhantomData<(&'a T, &'b U)>,
);

impl<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b> From<A> for Q<'a, 'b, N, T, U> {
    fn from(_value: A) -> Self {
        Self(PhantomData)
    }
}

#[test]
pub fn test_from() {
    let _ = A::from(D);
    let _ = B::from(D);
    let _ = Z::<()>::from(D);
    let _ = Y::from(D);
    let _ = W::<2>::from(D);
    let _ = Q::<2, (), ()>::from(D);
}
