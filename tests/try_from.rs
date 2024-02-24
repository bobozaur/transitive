mod macros;

use transitive::Transitive;

mod try_from_simple {
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
    #[transitive(from(ErrDC, ErrCB))] // impl From<ErrD_C> for ErrB_A
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

    #[test]
    pub fn test_try_from() {
        let _ = A::try_from(D);
        let _ = B::try_from(D);
    }
}

mod try_from_custom_err {
    use super::*;

    #[derive(Transitive)]
    #[transitive(try_from(D, C, B), error = "ConvErr")] // impl TryFrom<D> for A
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

    #[test]
    pub fn test_try_from_custom_err() {
        let _ = A::try_from(D);
    }
}
