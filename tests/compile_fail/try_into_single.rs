//! Test that `#[transitive(try_into(T))]` should fail to compile

fn main() {
    use std::convert::Infallible;
    use transitive::Transitive;
    
    struct A;

    struct B(A);

    impl TryFrom<B> for A {
        type Error = Infallible;

        fn try_from(outer: B) -> Result<Self, Self::Error> {
            Ok(outer.0)
        }
    }
    
    #[derive(Transitive)]
    #[transitive(try_into(A))] // impl From<C> for A
    struct C(B);

    impl TryFrom<C> for B {
        type Error = Infallible;

        fn try_from(outer: C) -> Result<Self, Self::Error> {
            Ok(outer.0)
        }
    }

    // Would cause an infinite loop
    let _: Result<A, <A as TryFrom<C>>::Error> = A::try_from(C(B(A)));
}
