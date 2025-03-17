//! Test that `#[transitive(try_from(T, error = Error))]` should fail to compile

fn main() {
    use std::convert::Infallible;
    use transitive::Transitive;
    
    struct A;

    struct B(A);

    impl TryFrom<A> for B {
        type Error = Infallible;

        fn try_from(inner: A) -> Result<Self, Self::Error> {
            Ok(Self(inner))
        }
    }
    
    #[derive(Transitive)]
    #[transitive(try_from(A, error = Infallible))] // impl From<A> for C
    struct C(B);

    impl TryFrom<B> for C {
        type Error = Infallible;

        fn try_from(inner: B) -> Result<Self, Self::Error> {
            Ok(Self(inner))
        }
    }

    // Would cause an infinite loop
    let _: Result<C, <C as TryFrom<A>>::Error> = C::try_from(A);
}
