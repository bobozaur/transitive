mod macros;

use transitive::Transitive;

#[derive(Transitive)]
#[transitive(into(B, C, D))] // impl From<A> for D
struct A;

#[derive(Transitive)]
#[transitive(into(C, D))] // impl From<B> for  D
struct B;
struct C;
struct D;

impl_from!(A to B);
impl_from!(B to C);
impl_from!(C to D);

#[test]
pub fn test_into() {
    let _ = D::from(A);
    let _ = D::from(B);
}
