//! Test that `#[transitive(into(T))]` should fail to compile

fn main() {
    use transitive::Transitive;
    
    struct A;

    struct B(A);

    impl From<B> for A {
        fn from(outer: B) -> Self {
            outer.0
        }
    }
    
    #[derive(Transitive)]
    #[transitive(into(A))] // impl From<C> for A
    struct C(B);

    impl From<C> for B {
        fn from(outer: C) -> Self {
            outer.0
        }
    }

    // Would cause an infinite loop
    let _: A = A::from(C(B(A)));
}
