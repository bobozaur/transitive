//! Test that `#[transitive(from(T))]` should fail to compile

fn main() {
    use transitive::Transitive;
    
    struct A;

    struct B(A);

    impl From<A> for B {
        fn from(inner: A) -> Self {
            Self(inner)
        }
    }
    
    #[derive(Transitive)]
    #[transitive(from(A))] // impl From<A> for C
    struct C(B);

    impl From<B> for C {
        fn from(inner: B) -> Self {
            Self(inner)
        }
    }

    // Would cause an infinite loop
    let _: C = C::from(A);
}
