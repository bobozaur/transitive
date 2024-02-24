mod macros;

use transitive::Transitive;

mod try_into_simple {
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

    #[test]
    pub fn test_try_into() {
        let _ = D::try_from(A);
        let _ = D::try_from(B);
    }
}

mod try_into_custom_err {
    use super::*;

    #[derive(Transitive)]
    #[transitive(try_into(B, C, D), error = "ConvErr")] // impl TryFrom<A> for D
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

    #[test]
    pub fn test_try_into_custom_err() {
        let _ = D::try_from(A);
    }
}
