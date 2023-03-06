# expect_with
[![Crates.io](https://img.shields.io/crates/v/expect-with.svg)](https://crates.io/crates/expect_with) [![Documentation](https://docs.rs/expect_with/badge.svg)](https://docs.rs/expect_with/)

Adds `expect_with()` for `Option<T>` and `Result<T, E>` where `E` is `Debug` (all std types that has `expect` method).

This method functions exactly the same as `except()` but evaluates error message only when actual error occurred.

## Example

```rust
fn some_heavy_function() -> String {
    return String::from("42");
}

let result: Result<(), String> = Err(String::from("some error"));
result.expect_with(|| format!("error {}", some_heavy_function()));
```

## Motivation

Using `expect` has one major drawback. It will calculate it's argument every time. Even if no error occurred. This can be really slow if `expect` is called frequently and evaluating error message envolves some computing (even simple `format` can be awfully slow). `expect_with` removes this overhead by accepting lambda, which will be executed to get error message only when needed.
