mod macros;

use transitive::Transitive;

#[derive(Transitive)]
#[transitive_from(path(D, C, B))] // impl From<D>for A
#[transitive_from(path(C, B))] // impl From<C> for A
#[transitive_try_into(path(B, C, D))] // impl TryFrom<A> for D
struct A;
struct B;
struct C;
struct D;

impl_from!(B to A);
impl_from!(C to B);
impl_from!(D to C);

impl_from!(A to B);
impl_from!(B to C);
impl_from!(C to D);

#[test]
pub fn test_combined_attributes() {
    let _ = A::from(D);
    let _ = A::from(C);
    let _ = D::try_from(A);
}
