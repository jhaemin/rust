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

### Blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions.

## Comments

All programmers strive to make their code easy to understand, but sometimes extra explanation is warranted.

## `if` Expressions

> `if` is an expression

### arms

Blocks of code associated with the conditions in `if` expressions are sometimes called **_arms_**, just like the arms in `match` expressions.

```rust
fn main() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  }
}
```

It's also worth nothing that the condition in this code _must_ be a `bool`. If the condition isn't a `bool`, we'll get an error.

```rust
fn main() {
  let number = 3;

  if number { // Error
    println!("number was three")'
  }
}
```

Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide `if` with a Boolean as its condition.

### The values that have the potential to be results from each arm of the `if` must be the same type

```rust
// Compile error

fn main() {
  let condition = true;

  let number = if condition {
    5 // Integer
  } else {
    "six" // String
  };
}
```

## `loop`

```rust
fn main() {
  loop {
    println("again");
  }
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
