# Turbo Genesis Macros

[![docs.rs](https://docs.rs/turbo-genesis-macros/badge.svg)](https://docs.rs/turbo-genesis-macros)
[![Crates.io](https://img.shields.io/crates/v/turbo-genesis-macros.svg)](https://crates.io/crates/turbo-genesis-macros)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Turbo logo banner](./banner.png)

Procedural macros for the TurboGenesis game runtime. These macros reduce boilerplate and expose intuitive annotations for defining game state, commands, channels, and program metadata compiled to WebAssembly.

---

## ✨ `#[game]`

Creates a WASM entrypoint with hot-reload support and runtime state persistence. Applies `#[turbo::serialize]` and generates:

- A `#[no_mangle] pub extern "C" fn run()` symbol
- Conditional branching for hot reload vs. static mode
- Automatic state (de)serialization and update loop

```rust
#[game]
pub struct MyGame {
    tick: u32,
}

impl MyGame {
    pub fn new() -> Self { Self { tick: 0 } }
    pub fn update(&mut self) { self.tick += 1; }
}
```

Requirements:

- `fn new() -> Self`
- `fn update(&mut self)`
- `BorshSerialize + BorshDeserialize`

---

## 🧬 `#[serialize]`

Applies `BorshSerialize`, `BorshDeserialize`, `serde::Serialize`, and `serde::Deserialize` to structs and enums.

```rust
#[serialize]
struct SaveData {
    level: u8,
    items: Vec<String>,
}
```

---

## 🎮 `#[command(name = "foo")]`

Registers a struct or enum as a Turbo command callable from clients. Adds:

- `exec()` method for client invocation
- Server entrypoint export

```rust
#[command(name = "greet")]
struct GreetCommand { user: String }

impl CommandHandler for GreetCommand {
    fn run(&mut self, user_id: &str) -> Result<(), std::io::Error> { ... }
}
```

---

## 📡 `#[channel(name = "chat")]`

Defines a duplex WebSocket-style handler for Turbo's channel system. Provides:

- An extern entrypoint for server dispatch
- A `subscribe()` method for clients

```rust
#[channel(name = "chat")]
struct ChatHandler;

impl ChannelHandler for ChatHandler {
    type Send = ChatLog;
    type Recv = ChatMessage;
    fn on_connect(&mut self, user_id: &str) { ... }
    fn on_data(&mut self, user_id: &str, msg: ChatMessage) { ... }
    fn on_interval(&mut self) { ... }
    fn on_close(&mut self) { ... }
}
```

---

## 📦 `#[program]`

Declares a module as a Turbo program, injects runtime constants, and calculates a stable program ID using the user UUID.

```toml
[package.metadata.turbo]
user = "<uuid>"
```

```rust
#[program]
mod my_game {
    // Now contains PROGRAM_ID, PROGRAM_NAME, etc.
}
```

Injects:

- `PROGRAM_NAME`, `PROGRAM_ID`, `PROGRAM_OWNER`
- `watch(path)` to observe reactive file changes

---

## 📚 Documentation

- [docs.rs/turbo-genesis-macros](https://docs.rs/turbo-genesis-macros)
- Part of the [TurboGenesis SDK](https://github.com/super-turbo-society/turbo-genesis-sdk)

## 📜 License

MIT License. See [LICENSE](LICENSE.md).
