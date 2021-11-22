
# Details

This section is about Object Oriented Programming https://doc.rust-lang.org/book/ch17-00-oop.html

# Usage

```
$ cargo build
$ cargo test
$ cargo run
```

## Notes

There is a small section talking about the performance impact of dynamic dispatch. Rust needs to figure which object to call at runtime, as opposed to having the type at compile time. Here is an interesting read where a blogger ran some tests https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b

  tl;dr it can be up to 3x slower, but also have little to no impact
  when you give the compiler a chance to figure out the type.

Also found an interesting command line benchmarking tool https://github.com/sharkdp/hyperfine

https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
https://doc.rust-lang.org/reference/items/traits.html#object-safety

Specific traits in Traits which creates object safety. For example, Clone is 
not safe because it returns Self. Traits need to be more precise on their types.
