# Cargo

Cargo is a Rust build system and package manager.

## Create
```
$ cargo new hello_cargo
$ cd hello_cargo
$ cargo check
```
## Build

```
$ cargo build
```
## Run

```
$ ./target/debug/hello_cargo
$ cargo run
```
## Integration tests

This integrates with the hello_crate library.

```
$ cargo test
```

## Useful tools

```
$ cargo fmt
$ cargo fix
$ cargo clippy
```

More on cargo fmt.
https://rust-lang.github.io/rustfmt/
https://marketplace.visualstudio.com/items?itemName=statiolake.vscode-rustfmt

There is a good integration tool with vscode and rustfmt. In your cargo package
you can specify a rustfmt.toml|.rustfmt.toml file. vscode reads it and applies
the formatting rules to your editor. Very simple!

## Adding dependencies

Cargo does not have an add command. But if you have cargo-edit there is.
You can also manage your Cargo.toml directly.

```
$ cargo add plotters
PS C:\Users\Travis\Development\rust\adb-sim\cli> cargo add plotters
error: no such subcommand: `add`

        Did you mean `d`?
```

```
$ cargo install cargo-edit
$ cargo add plotters
```

## Documentation

```
$ cargo doc
$ cargo doc --open
```
