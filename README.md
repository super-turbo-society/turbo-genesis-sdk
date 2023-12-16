# Turbo Genesis SDK

See more documentation at https://docs.rs/turbo-genesis-sdk/latest/turbo_genesis_sdk/

## Getting Started

### Create a project

First, create a project:

```sh
cargo init --lib your-project-name
```

### Add the dependency

Add the sdk as a dependency in `Cargo.toml`:

```sh
[dependencies]
turbo = { package = "turbo-genesis-sdk" }
```

### Update your code

Open `src/lib.rs` and add the following:

```rs
turbo::go! {
    text!("Hello, world!!!");
}
```

### Build

Build with the following command:

```sh
cargo build --target wasm32-unknown-unknown
```

Run your game with [Turbo](https://docs.turbo.computer)

```sh
turbo-cli run -w .
```