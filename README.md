[![Crates.io](https://img.shields.io/crates/v/transitive.svg)](https://crates.io/crates/transitive)

# transitive
Transitive converions through derive macros for Rust.

## Rationale:
Assume you have types `A`, `B` and `C` with the following, already implemented, conversions:
- `A -> B`
- `B -> C`

Sometimes it might be desirable to have an `A -> C` implementation which could easily be represented as `A -> B -> C`.

That is precisely what this crate does. Through the `Transitive` derive macro, it will implement `From` or `TryFrom` respectively
for converting from/to the derived type and a target type, given a path of transitions to go through.

```rust
use transitive::Transitive;

#[derive(Transitive)]
#[transitive(into(B, C, D))] // impl From<A> for D by doing A -> B -> C -> D
struct A;

#[derive(Transitive)]
#[transitive(into(C, D))] // impl From<B> for D by doing B -> C -> D
struct B;
struct C;
struct D;

impl From<A> for B {
    fn from(val: A) -> Self {
        Self
    }
};

impl From<B> for C {
    fn from(val: B) -> Self {
        Self
    }
};

impl From<C> for D {
    fn from(val: C) -> Self {
        Self
    }
};

#[test]
fn into() {
    D::from(A);
    D::from(B);
}
```

More examples and explanations can be found in the [documentation](https://docs.rs/transitive/latest/transitive/).

## License
Licensed under MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT).

## Contributing
Contributions to this repository, unless explicitly stated otherwise, will be considered licensed under MIT.
Bugs/issues encountered can be opened [here](https://github.com/bobozaur/transitive/issues).