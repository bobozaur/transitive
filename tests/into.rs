mod macros;

use std::marker::PhantomData;

use transitive::Transitive;

#[derive(Transitive)]
#[transitive_into(path(B, C, D))] // impl From<A> for D
struct A;

#[derive(Transitive)]
#[transitive_into(path(C, D))] // impl From<B> for D
struct B;
struct C;
struct D;

impl_from!(A to B);
impl_from!(B to C);
impl_from!(C to D);

#[derive(Transitive)]
#[transitive_into(path(A, B, C, D))] // impl From<Z<T>> for D
struct Z<T>(PhantomData<T>);

impl<T> From<Z<T>> for A {
    fn from(_value: Z<T>) -> Self {
        Self
    }
}

#[derive(Transitive)]
#[transitive_into(path(A, B, C, D))] // impl From<Y<'a>> for D
struct Y<'a>(PhantomData<&'a ()>);

impl<'a> From<Y<'a>> for A {
    fn from(_value: Y<'a>) -> Self {
        Self
    }
}

#[derive(Transitive)]
#[transitive_into(path(A, B, C, D))] // impl From<W<N>> for D
struct W<const N: usize>;

impl<const N: usize> From<W<N>> for A {
    fn from(_value: W<N>) -> Self {
        Self
    }
}

#[derive(Transitive)]
#[transitive_into(path(A, B, C, D))] // impl From<Q<'a, 'b, N, T, U>> for D
struct Q<'a, 'b: 'a, const N: usize, T: 'a + Send, U: 'b = &'b str>(PhantomData<(&'a T, &'b U)>)
where
    T: Sync;

impl<'a, 'b: 'a, const N: usize, T: 'a + Send, U: 'b> From<Q<'a, 'b, N, T, U>> for A
where
    T: Sync,
{
    fn from(_value: Q<'a, 'b, N, T, U>) -> Self {
        Self
    }
}

#[test]
pub fn test_into() {
    let _ = D::from(A);
    let _ = D::from(B);
    let _ = D::from(Z(PhantomData::<()>));
    let _ = D::from(Y(PhantomData));
    let _ = D::from(W::<2>);
    let _ = D::from(Q::<2, (), ()>(PhantomData));
}
