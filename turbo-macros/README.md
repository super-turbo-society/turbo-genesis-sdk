# Turbo Macros

[![docs.rs](https://docs.rs/turbo-genesis-sdk/badge.svg)](https://docs.rs/turbo-genesis-sdk)
[![Crates.io](https://img.shields.io/crates/v/turbo-genesis-sdk.svg)](https://crates.io/crates/turbo-genesis-sdk)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Turbo logo banner](./banner.png)

This crate defines a procedural macros for use in Turbo ecosystem crates.

## ✨ `#[game]`

The proc macro attribute `#[game]` for simplifying state management and FFI bindings in Turbo-powered games compiled to WebAssembly.

The `#[game]` macro decorates a `struct` or `enum` representing your game's top-level state. It generates a `#[no_mangle] pub extern "C" fn run()` entrypoint for WASM. This function handles:

- Deserializing state from Turbo's runtime storage (in hot-reload mode).
- Serializing state after each game loop (in hot-reload mode).
- Calling `.update()` on the game state struct or enum each frame.

This ensures minimal boilerplate in user-facing game code.

### Example

This macro is exported from the [`Turbo SDK`](https://github.com/super-turbo-society/turbo-genesis-sdk) via the `use turbo::*` statement. Usage is as follows:

```rust
use turbo::*;
use borsh::{BorshSerialize, BorshDeserialize};

#[game]
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct MyGame {
    tick: u32,
}
impl MyGame {
    // This method will initialize your game state during the first frame.
    pub fn new() -> Self {
        Self { tick: 0 }
    }
    // This method runs each frame. Use it to update state and draw graphics.
    pub fn update(&mut self) {
        self.tick += 1;
    }
}
```

### Notes

#### 🔧 Requirements

Your struct must implement the following methods:

- `fn new() -> Self`
- `fn update(&mut self)`

Your struct must implement the following traits:

- `BorshSerialize`
- `BorshDeserialize`

#### 🏴‍☠️ Compilation

This macro compiles conditionally depending on whether or not hot-reloading is enabled.

| Flag                            | Behavior                                                            |
| ------------------------------- | ------------------------------------------------------------------- |
| `#[cfg(turbo_hot_reload)]`      | Loads state from `turbo::sys::load()` each run, saves after update. |
| `#[cfg(not(turbo_hot_reload))]` | Stores persistent state in a `static mut Option<T>`.                |

## 📖 Documentation

- Full API docs on [docs.rs/turbo-macros](https://docs.rs/turbo-macros)

## 📜 License

This project is licensed under MIT. See [LICENSE](LICENSE.md) for details.