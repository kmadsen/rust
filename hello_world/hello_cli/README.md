## Grrs - basic grep clone

https://rust-cli.github.io/book/tutorial/setup.html

### Usage

```
cd grrs
cargo build
cargo test
cargo run <pattern> <path>
```

Example
```
rust\hello_world\hello_cli\grrs> cargo run duplicate .\resources\tests\find_matches.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target\debug\grrs.exe duplicate .\resources\tests\find_matches.txt`
duplicate
duplicate
```
