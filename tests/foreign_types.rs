mod macros;

use std::num::ParseIntError;

use transitive::Transitive;

#[derive(Transitive)]
#[transitive_try_from(path(u8, C, B))] // impl TryFrom<u8> for A
struct A;

#[derive(Transitive)]
#[transitive_try_from(path(u8, C))] // impl TryFrom<u8> for B
struct B;
struct C;

struct ErrCB;
#[derive(Transitive)]
#[transitive_from(path(ParseIntError, ErrCB))] // impl From<ParseIntError> for ErrB_A
struct ErrBA;

impl From<ParseIntError> for ErrCB {
    fn from(_value: ParseIntError) -> Self {
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
impl_try_from!(u8 to C err ParseIntError);

#[test]
pub fn test_foreign_types() {
    let _ = A::try_from(1);
    let _ = B::try_from(1);
}
