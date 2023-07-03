use transitive::Transitive;

use crate::impl_try_from;

pub use try_from_custom_err::try_from_custom_err;
pub use try_from_simple::try_from;

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

    struct ErrD_C;
    struct ErrC_B;

    #[derive(Transitive)]
    #[transitive(from(ErrD_C, ErrC_B))] // impl From<ErrD_C> for ErrB_A
    struct ErrB_A;

    impl From<ErrD_C> for ErrC_B {
        fn from(value: ErrD_C) -> Self {
            Self
        }
    }

    impl From<ErrC_B> for ErrB_A {
        fn from(value: ErrC_B) -> Self {
            Self
        }
    }

    impl_try_from!(B to A err ErrB_A);
    impl_try_from!(C to B err ErrC_B);
    impl_try_from!(D to C err ErrD_C);

    pub fn try_from() {
        A::try_from(D);
        B::try_from(D);
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

    struct ErrD_C;
    struct ErrC_B;
    struct ErrB_A;

    impl From<ErrD_C> for ConvErr {
        fn from(value: ErrD_C) -> Self {
            Self
        }
    }

    impl From<ErrC_B> for ConvErr {
        fn from(value: ErrC_B) -> Self {
            Self
        }
    }

    impl From<ErrB_A> for ConvErr {
        fn from(value: ErrB_A) -> Self {
            Self
        }
    }

    impl_try_from!(B to A err ErrB_A);
    impl_try_from!(C to B err ErrC_B);
    impl_try_from!(D to C err ErrD_C);

    pub fn try_from_custom_err() {
        A::try_from(D);
    }
}
