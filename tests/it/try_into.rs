use transitive::Transitive;

use crate::impl_try_from;

pub use try_into_custom_err::try_into_custom_err;
pub use try_into_simple::try_into;

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

    struct ErrA_B;
    struct ErrB_C;
    #[derive(Transitive)]
    #[transitive(from(ErrA_B, ErrB_C))] // impl From<ErrA_B> for ErrC_D
    struct ErrC_D;

    impl From<ErrA_B> for ErrB_C {
        fn from(value: ErrA_B) -> Self {
            Self
        }
    }

    impl From<ErrB_C> for ErrC_D {
        fn from(value: ErrB_C) -> Self {
            Self
        }
    }

    impl_try_from!(A to B err ErrA_B);
    impl_try_from!(B to C err ErrB_C);
    impl_try_from!(C to D err ErrC_D);

    pub fn try_into() {
        D::try_from(A);
        D::try_from(B);
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
    struct ErrA_B;
    struct ErrB_C;
    struct ErrC_D;

    impl From<ErrA_B> for ConvErr {
        fn from(value: ErrA_B) -> Self {
            Self
        }
    }

    impl From<ErrB_C> for ConvErr {
        fn from(value: ErrB_C) -> Self {
            Self
        }
    }

    impl From<ErrC_D> for ConvErr {
        fn from(value: ErrC_D) -> Self {
            Self
        }
    }

    impl_try_from!(A to B err ErrA_B);
    impl_try_from!(B to C err ErrB_C);
    impl_try_from!(C to D err ErrC_D);

    pub fn try_into_custom_err() {
        D::try_from(A);
    }
}
