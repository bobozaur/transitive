mod macros;

use transitive::Transitive;

mod try_from_simple {
    use std::marker::PhantomData;

    use super::*;

    #[derive(Transitive)]
    #[transitive_try_from(path(D, C, B))] // impl TryFrom<D> for A
    struct A;

    #[derive(Transitive)]
    #[transitive_try_from(path(D, C))] // impl TryFrom<D> for B
    struct B;
    struct C;
    struct D;

    struct ErrDC;
    struct ErrCB;

    #[derive(Transitive)]
    #[transitive_from(path(ErrDC, ErrCB))] // impl From<ErrDC> for ErrBA
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
    #[transitive_try_from(path(D, C, B, A))] // impl TryFrom<D> for Z<T>
    struct Z<T>(PhantomData<T>);

    impl<T> TryFrom<A> for Z<T> {
        type Error = ErrBA;

        fn try_from(_value: A) -> Result<Self, Self::Error> {
            Ok(Self(PhantomData))
        }
    }

    #[derive(Transitive)]
    #[transitive_try_from(path(D, C, B, A))] // impl TryFrom<D> for Y<'a>
    struct Y<'a>(PhantomData<&'a ()>);

    impl<'a> TryFrom<A> for Y<'a> {
        type Error = ErrBA;

        fn try_from(_value: A) -> Result<Self, Self::Error> {
            Ok(Self(PhantomData))
        }
    }

    #[derive(Transitive)]
    #[transitive_try_from(path(D, C, B, A))] // impl TryFrom<D> for W<N>
    struct W<const N: usize>;

    impl<const N: usize> TryFrom<A> for W<N> {
        type Error = ErrBA;

        fn try_from(_value: A) -> Result<Self, Self::Error> {
            Ok(Self)
        }
    }

    #[derive(Transitive)]
    #[transitive_try_from(path(D, C, B, A))] // impl TryFrom<D> for Q<'a, 'b, N, T, U>
    struct Q<'a, 'b: 'a, const N: usize, T: 'a + Send, U: 'b>(PhantomData<(&'a T, &'b U)>)
    where
        T: Sync;

    impl<'a, 'b: 'a, const N: usize, T: 'a + Send, U: 'b> TryFrom<A> for Q<'a, 'b, N, T, U>
    where
        T: Sync,
    {
        type Error = ErrBA;

        fn try_from(_value: A) -> Result<Self, Self::Error> {
            Ok(Self(PhantomData))
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
    }
}

mod try_from_custom_err {
    use std::marker::PhantomData;

    use super::*;

    #[derive(Transitive)]
    #[transitive_try_from(path(D, C, B), error = "ConvErr")] // impl TryFrom<D> for A
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
    #[transitive_try_from(path(D, C, B, A), error = "ConvErr")] // impl TryFrom<D> for Q<'a, 'b, N, T, U>
    struct Q<'a, 'b: 'a, const N: usize, T: 'a + Send, U: 'b>(PhantomData<(&'a T, &'b U)>)
    where
        T: Sync;

    impl<'a, 'b: 'a, const N: usize, T: 'a + Send, U: 'b> TryFrom<A> for Q<'a, 'b, N, T, U>
    where
        T: Sync,
    {
        type Error = ConvErr;

        fn try_from(_value: A) -> Result<Self, Self::Error> {
            Ok(Self(PhantomData))
        }
    }

    #[test]
    pub fn test_try_from_custom_err() {
        let _ = A::try_from(D);
        let _ = Q::<2, (), ()>::try_from(D);
    }
}
