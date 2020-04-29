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

## Ownership

### Ownership Rules

First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

**Example**

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

- When s comes into scope, it is valid.
- It remains valid until it goes out of scope.

### Ways Variables and Data Interact: Move

```rust
let x = 5;
let y = x;
```

`x` and `y` are both valid.

```rust
let s1 = String::from("hello");
let s2 = s1;
```

`s1` was moved into `s2`. So `s1` is invalid.

### Ways Variables and Data Interact: Clone

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

### Stack-Only Data: Copy

### ❗️Slice Type

> Need more investigation

## Structs

```rs
#[derive(Debug)] // For debug print (optional)
struct Rectangle {
  width: u32,
  height: u32,
}
```

### Methods

Similar to functions but implemented in structs themselves. Their first parameter is always `self`, which represents the instance of the struct the method is being called on.

```rs
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}
```

**Associated Functions**

Another useful feature of impl blocks is that we’re allowed to define functions within impl blocks that don’t take self as a parameter. Associated functions are often used for constructors that will return a new instance of the struct.

```rs
impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

// Inside main
let rect = Rectangle::square(311); // Create a 311 x 311 rectangle
```

## Enums

```rs
enum IpAddrKind {
  V4,
  V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

fn route(ip_kind: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

```rs
enum IpAddr {
  V4(String),
  V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

```rs
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

### `Option`

**_Rust doesn’t have the null feature_** that many other languages have. Null is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.

In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, has this to say:

> I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.

```rs
enum Option<T> {
    Some(T),
    None,
}
```

```rs
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

- **Some**: We know that a value is present and the value is held within the `Some`.
- **None**: In some sense, it means the same thing as null: we don't have a valid value.

```rs
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y; // Error
```

When we have a value of a type like `i8` in Rust, the compiler will ensure that **_we always have a valid value_**. We can proceed confidently without having to check for null before using that value.

Only when we have an `Option<i8>` (or whatever type of value we're working with) do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value.

### `match`

```rs
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
```

**Matches are exhaustive**

Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid. Especially in the case of `Option<T>`, when Rust prevents us from forgetting to explicitly handle the None case, it protects us from assuming that we have a value when we might have null, thus making the billion-dollar mistake discussed earlier.

**The `_` placeholder**

```rs
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
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
