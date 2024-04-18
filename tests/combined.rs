mod macros;

use transitive::Transitive;

#[derive(Transitive)]
#[transitive(from(D, C, B))] // impl From<D>for A
#[transitive(from(C, B))] // impl From<C> for A
#[transitive(try_into(B, C, D))] // impl TryFrom<A> for D
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
