mod macros;

use transitive::Transitive;

mod try_into_simple {
    use std::{convert::Infallible, marker::PhantomData};

    use super::*;

    #[derive(Transitive)]
    #[transitive(try_into(B, C, D))] // impl TryFrom<A> for D
    struct A;

    #[derive(Transitive)]
    #[transitive(try_into(C, D))] // impl TryFrom<B> for  D
    struct B;
    struct C;
    struct D;

    struct ErrAB;
    struct ErrBC;
    #[derive(Transitive)]
    #[transitive(from(ErrAB, ErrBC))] // impl From<ErrAB> for ErrCD
    struct ErrCD;

    impl From<ErrAB> for ErrBC {
        fn from(_value: ErrAB) -> Self {
            Self
        }
    }

    impl From<ErrBC> for ErrCD {
        fn from(_value: ErrBC) -> Self {
            Self
        }
    }

    impl_try_from!(A to B err ErrAB);
    impl_try_from!(B to C err ErrBC);
    impl_try_from!(C to D err ErrCD);

    #[derive(Transitive)]
    #[transitive(try_into(A, B, C, D))] // impl TryFrom<Z<T>> for D
    struct Z<T>(PhantomData<T>);

    impl<T> TryFrom<Z<T>> for A {
        type Error = ErrAB;

        fn try_from(_value: Z<T>) -> Result<Self, Self::Error> {
            Ok(Self)
        }
    }

    #[derive(Transitive)]
    #[transitive(try_into(A, B, C, D))] // impl TryFrom<Y<'a>> for D
    struct Y<'a>(PhantomData<&'a ()>);

    impl<'a> TryFrom<Y<'a>> for A {
        type Error = ErrAB;

        fn try_from(_value: Y<'a>) -> Result<Self, Self::Error> {
            Ok(Self)
        }
    }

    #[derive(Transitive)]
    #[transitive(try_into(A, B, C, D))] // impl TryFrom<W<N>> for D
    struct W<const N: usize>;

    impl<const N: usize> TryFrom<W<N>> for A {
        type Error = ErrAB;

        fn try_from(_value: W<N>) -> Result<Self, Self::Error> {
            Ok(Self)
        }
    }

    #[derive(Transitive)]
    #[transitive(try_into(A, B, C, D))] // impl TryFrom<Q<'a, 'b, N, T, U>> for D
    struct Q<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b = &'b str>(
        PhantomData<(&'a T, &'b U)>,
    );

    impl<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b> TryFrom<Q<'a, 'b, N, T, U>> for A {
        type Error = ErrAB;

        fn try_from(_value: Q<'a, 'b, N, T, U>) -> Result<Self, Self::Error> {
            Ok(Self)
        }
    }

    #[derive(Transitive)]
    #[transitive(try_into(A, Q<'static, 'static, 2, () ,()>))] // impl TryFrom<G> for Q<'static, 'static, 2, (), ()>
    struct G;

    impl<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b> TryFrom<A> for Q<'a, 'b, N, T, U> {
        type Error = Infallible;

        fn try_from(_: A) -> Result<Self, Self::Error> {
            Ok(Q(PhantomData))
        }
    }

    impl TryFrom<G> for A {
        type Error = Infallible;

        fn try_from(_: G) -> Result<Self, Self::Error> {
            Ok(A)
        }
    }

    #[test]
    pub fn test_try_into() {
        let _ = D::try_from(A);
        let _ = D::try_from(B);
        let _ = D::try_from(Z(PhantomData::<()>));
        let _ = D::try_from(Y(PhantomData));
        let _ = D::try_from(W::<2>);
        let _ = D::try_from(Q::<2, (), ()>(PhantomData));
        let _ = Q::try_from(G);
    }
}

mod try_into_custom_err {
    use std::marker::PhantomData;

    use super::*;

    #[derive(Transitive)]
    #[transitive(try_into(B, C, D, error = ConvErr))] // impl TryFrom<A> for D
    struct A;
    struct B;
    struct C;
    struct D;

    struct ConvErr;
    struct ErrAB;
    struct ErrBC;
    struct ErrCD;

    impl From<ErrAB> for ConvErr {
        fn from(_value: ErrAB) -> Self {
            Self
        }
    }

    impl From<ErrBC> for ConvErr {
        fn from(_value: ErrBC) -> Self {
            Self
        }
    }

    impl From<ErrCD> for ConvErr {
        fn from(_value: ErrCD) -> Self {
            Self
        }
    }

    impl_try_from!(A to B err ErrAB);
    impl_try_from!(B to C err ErrBC);
    impl_try_from!(C to D err ErrCD);

    #[derive(Transitive)]
    #[transitive(try_into(A, B, C, D, error = ConvErr))] // impl TryFrom<Q<'a, 'b, N, T, U>> for D
    struct Q<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b, V = String>(
        PhantomData<(&'a T, &'b U, V)>,
    );

    impl<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b, V> TryFrom<Q<'a, 'b, N, T, U, V>>
        for A
    {
        type Error = ConvErr;

        fn try_from(_value: Q<'a, 'b, N, T, U, V>) -> Result<Self, Self::Error> {
            Ok(Self)
        }
    }

    #[derive(Transitive)]
    #[transitive(try_into(A, Q<'static, 'static, 2, () ,()>))] // impl TryFrom<G> for Q<'static, 'static, 2, (), ()>
    struct G;

    impl<'a, 'b: 'a, const N: usize, T: 'a + Send + Sync, U: 'b> TryFrom<A> for Q<'a, 'b, N, T, U> {
        type Error = ConvErr;

        fn try_from(_: A) -> Result<Self, Self::Error> {
            Ok(Q(PhantomData))
        }
    }

    impl TryFrom<G> for A {
        type Error = ConvErr;

        fn try_from(_: G) -> Result<Self, Self::Error> {
            Ok(A)
        }
    }

    #[test]
    pub fn test_try_into_custom_err() {
        let _ = D::try_from(A);
        let _ = D::try_from(Q::<2, (), ()>(PhantomData));
        let _ = Q::try_from(G);
    }
}
