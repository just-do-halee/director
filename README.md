# **`Director`**

`Director` is a simple, versatile, ergonomic state machine in Rust-lang.

[![CI][ci-badge]][ci-url]
[![Crates.io][crates-badge]][crates-url]
[![Licensed][license-badge]][license-url]
[![Twitter][twitter-badge]][twitter-url]

[ci-badge]: https://github.com/just-do-halee/director/actions/workflows/ci.yml/badge.svg
[crates-badge]: https://img.shields.io/crates/v/director.svg?labelColor=383636
[license-badge]: https://img.shields.io/crates/l/director?labelColor=383636
[twitter-badge]: https://img.shields.io/twitter/follow/do_halee?style=flat&logo=twitter&color=4a4646&labelColor=333131&label=just-do-halee
[ci-url]: https://github.com/just-do-halee/director/actions
[twitter-url]: https://twitter.com/do_halee
[crates-url]: https://crates.io/crates/director
[license-url]: https://github.com/just-do-halee/director

| [Examples](./examples/) | [Docs](https://docs.rs/director) | [Latest Note](./CHANGELOG.md) |

```toml
director = "0.3"
# { default-features = false } is for no_std
```

## **`Why?`**

Because writing state-machine is kind of tedious. Not for human way. It is difficult to achieve flexibility, readability and more analyzability to the architecture. [This crate](https://crates.io/crates/director) gives all of them. And well optimized[ex:) RAII]. So you don't need to worry way bad performance for your implementation.

## **`How to use,`**

```rust
use crate::Engine; // Any common state

#[director::state {
    super = StateBaz,
    sub = [StateBar, StateBar2]
}]
pub struct StateFoo {
    count: u32,
}

impl director::State<Engine> for StateFoo {
    /// This determines whether or not to run this local state machine.
    fn toggle(engine: &mut Engine, inner: Option<&Self>) -> bool {
        director::on!(inner, None) || director::off!(state: StateBaz, Some(state) if state.count > 1000)
    }
    /// This creates and imports new initial state on this local state machine when the toggle's on.
    fn load(engine: &mut Engine) -> Self {
        Self { count: Self::lock_super__state_baz().get().count }
    }
    /// This executes custom logics and manipulates this local state machine's states.
    fn run(&mut self, engine: &mut Engine) {
        self.count += 1;
        println!("{}", self.count);
    }
    /// When the toggle's off
    fn drop(&self, engine: &mut Engine) {
        // ...
        // After then, the sub states[i.e) StateBar and StateBar2] will be droped automatically.
    }
}
```

```rust
pub struct Engine; // i.e) dummy engine

#[director::main(std::syn)] // It can be any kind of Mutex
fn main() {
    for _ in 0..10000 {
        StateBaz::run(&mut engine);
    }
}
```
