mod macros;

use transitive::Transitive;

mod try_from_simple {
    use std::{convert::Infallible, marker::PhantomData};

    use super::*;

    #[derive(Transitive)]
    #[transitive(try_from(D, C, B))] // impl TryFrom<D> for A
    struct A;

    #[derive(Transitive)]
    #[transitive(try_from(D, C))] // impl TryFrom<D> for B
    struct B;
    struct C;
    struct D;

    struct ErrDC;
    struct ErrCB;

    #[derive(Transitive)]
    #[transitive(from(ErrDC, ErrCB))] // impl From<ErrDC> for ErrBA
    struct ErrBA;

    impl From<ErrDC> for ErrCB {
        fn from(_value: ErrDC) -> Self {
            Self
        }
    }

    impl From<ErrCB> for ErrBA {
        fn from(_value: ErrCB) -> Self {
            Self
        }
    }

    impl_try_from!(B to A err ErrBA);
    impl_try_from!(C to B err ErrCB);
    impl_try_from!(D to C err ErrDC);

    #[derive(Transitive)]
    #[transitive(try_from(D, C, B, A))] // impl TryFrom<D> for Z<T>
    struct Z<T>(PhantomData<T>);

    impl<T> TryFrom<A> for Z<T> {
        type Error = ErrBA;

        fn try_from(_value: A) -> Result<Self, Self::Error> {
            Ok(Self(PhantomData))
        }
    }

    #[derive(Transitive)]
    #[transitive(try_from(D, C, B, A))] // impl TryFrom<D> for Y<'a>
    struct Y<'a>(PhantomData<&'a ()>);

    impl TryFrom<A> for Y<'_> {
        type Error = ErrBA;

        fn try_from(_value: A) -> Result<Self, Self::Error> {
            Ok(Self(PhantomData))
        }
    }

    #[derive(Transitive)]
    #[transitive(try_from(D, C, B, A))] // impl TryFrom<D> for W<N>
    struct W<const N: usize>;

    impl<const N: usize> TryFrom<A> for W<N> {
        type Error = ErrBA;

        fn try_from(_value: A) -> Result<Self, Self::Error> {
            Ok(Self)
        }
    }

    #[derive(Transitive)]
    #[transitive(try_from(D, C, B, A))] // impl TryFrom<D> for Q<'a, 'b, N, T, U>
    struct Q<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b>(PhantomData<(&'a T, &'b U)>);

    impl<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b> TryFrom<A> for Q<'a, 'b, N, T, U> {
        type Error = ErrBA;

        fn try_from(_value: A) -> Result<Self, Self::Error> {
            Ok(Self(PhantomData))
        }
    }

    #[derive(Transitive)]
    #[transitive(try_from(Q<'static, 'static, 2, () ,()>, A))] // impl TryFrom<Q<'static, 'static, 2, (), ()>> for G
    struct G;

    impl<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b> TryFrom<Q<'a, 'b, N, T, U>> for A {
        type Error = Infallible;

        fn try_from(_: Q<'a, 'b, N, T, U>) -> Result<Self, Self::Error> {
            Ok(A)
        }
    }

    impl TryFrom<A> for G {
        type Error = Infallible;

        fn try_from(_: A) -> Result<Self, Self::Error> {
            Ok(G)
        }
    }

    #[derive(Transitive)]
    #[transitive(try_from(Q<'a, 'static, 2, T ,()>, A))] // impl TryFrom<Q<'a, 'static, 2, T, ()>> for P<'a, T>
    struct P<'a, T: Send + Sync>(PhantomData<fn() -> &'a T>);

    impl<T: Send + Sync> TryFrom<A> for P<'_, T> {
        type Error = Infallible;

        fn try_from(_: A) -> Result<Self, Self::Error> {
            Ok(P(PhantomData))
        }
    }

    #[test]
    pub fn test_try_from() {
        let _ = A::try_from(D);
        let _ = B::try_from(D);
        let _ = Z::<()>::try_from(D);
        let _ = Y::try_from(D);
        let _ = W::<2>::try_from(D);
        let _ = Q::<2, (), ()>::try_from(D);
        let _ = G::try_from(Q::<'static, 'static, 2, (), ()>(PhantomData));
        let _ = P::try_from(Q::<'static, 'static, 2, (), ()>(PhantomData));
    }
}

mod try_from_custom_err {
    use std::marker::PhantomData;

    use super::*;

    #[derive(Transitive)]
    #[transitive(try_from(D, C, B, error = ConvErr))] // impl TryFrom<D> for A
    struct A;
    struct B;
    struct C;
    struct D;

    struct ConvErr;

    struct ErrDC;
    struct ErrCB;
    struct ErrBA;

    impl From<ErrDC> for ConvErr {
        fn from(_value: ErrDC) -> Self {
            Self
        }
    }

    impl From<ErrCB> for ConvErr {
        fn from(_value: ErrCB) -> Self {
            Self
        }
    }

    impl From<ErrBA> for ConvErr {
        fn from(_value: ErrBA) -> Self {
            Self
        }
    }

    impl_try_from!(B to A err ErrBA);
    impl_try_from!(C to B err ErrCB);
    impl_try_from!(D to C err ErrDC);

    #[derive(Transitive)]
    #[transitive(try_from(D, C, B, A, error = ConvErr))] // impl TryFrom<D> for Q<'a, 'b, N, T, U>
    struct Q<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b = &'b str>(
        PhantomData<(&'a T, &'b U)>,
    );

    impl<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b> TryFrom<A> for Q<'a, 'b, N, T, U> {
        type Error = ConvErr;

        fn try_from(_value: A) -> Result<Self, Self::Error> {
            Ok(Self(PhantomData))
        }
    }

    #[derive(Transitive)]
    #[transitive(try_from(Q<'static, 'static, 2, () ,()>, A, error = ConvErr))] // impl TryFrom<Q<'static, 'static, 2, (), ()>> for G
    struct G;

    impl<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b> TryFrom<Q<'a, 'b, N, T, U>> for A {
        type Error = ConvErr;

        fn try_from(_: Q<'a, 'b, N, T, U>) -> Result<Self, Self::Error> {
            Ok(A)
        }
    }

    impl TryFrom<A> for G {
        type Error = ConvErr;

        fn try_from(_: A) -> Result<Self, Self::Error> {
            Ok(G)
        }
    }

    #[derive(Transitive)]
    #[transitive(try_from(Q<'a, 'static, 2, T ,()>, A))] // impl TryFrom<Q<'a, 'static, 2, T, ()>> for P<'a, T>
    struct P<'a, T: Send + Sync>(PhantomData<fn() -> &'a T>);

    impl<T: Send + Sync> TryFrom<A> for P<'_, T> {
        type Error = ConvErr;

        fn try_from(_: A) -> Result<Self, Self::Error> {
            Ok(P(PhantomData))
        }
    }

    #[test]
    pub fn test_try_from_custom_err() {
        let _ = A::try_from(D);
        let _ = Q::<2, (), ()>::try_from(D);
        let _ = G::try_from(Q::<'static, 'static, 2, (), ()>(PhantomData));
        let _ = P::try_from(Q::<'static, 'static, 2, (), ()>(PhantomData));
    }
}
