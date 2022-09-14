# **`Director`**

`Director` is a simple, versatile, ergonomic state machine in Rust-lang.

<br>

```toml
director = "0.1.0"
```

<br>

## **`Getting Started`**

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
        Self { count: Self::lock_sup__state_baz().get().count }
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

fn main() {
    StateFoo::run(&mut engine);
}
```
