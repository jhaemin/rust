# Rust

This repository demonstrates my learning process of **Rust** programming language from the ground up, following the [official guide book](https://doc.rust-lang.org/stable/book) step by step.

## Statements vs Expressions

Rust is an expression-based language, so it is important to understand its definition correctly.

- **_Statements_** are instructions that perform some actions and do not return a value.
- **_Expressions_** evaluate to a resulting value.

```rust
fn main() {
  let y = 6; // statement
  y + 1 // expression: resolves to 7
  y + 1; // statement: does not return any value
}
```

> Unlike other languages, such as C and Ruby, where the assignment returns the value of the assignment, Rust does not.

### No `;` at the end of an expression

Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.

### Functions with Return Values

In Rust, the return value of the function is synonymous with the value of the <u>final expression</u> in the block of the body of a function. You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly.

```rust
fn ten() -> i32 {
  10
}

fn plus_one(x: i32) -> i32 {
  x + 1
}

fn main() {
  let x = plus_one(2);

  println!("{}", x); // "3"
}

fn minus_one(x: i32) -> i32 {
  x - 1; // compile error: because it is a statement
}
```

## Cheatsheet

### Commands

```zsh
rustup update
cargo --version
cargo build
cargo build --release
cargo check
cargo run
cargo doc --open
```

### Keywords

- `let`
- `mut`
- `const`
- `match`
- `loop`
