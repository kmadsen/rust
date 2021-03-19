# rustup rustc

rustup manages Rust updates and rustc the Rust compiler

$ rustup docs --book

## Install, verify, uptate

$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ rustc --version
$ rustup update

## Build and run

$ rustc main.rs
$ ./main
Hello, world!

# Cargo

Cargo is a Rust build system and package manager.

## Create, build, run

$ cargo new hello_cargo
$ cd hello_cargo
$ cargo check
$ cargo build
$ ./target/debug/hello_cargo
$ cargo run
