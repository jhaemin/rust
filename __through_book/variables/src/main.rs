fn variable() {
    // A variable is immutable in default.
    // Use `mut` keyword to make a variable mutable.
    let mut x = 5;
    println!("x: {}", x);
    x = 6;
    println!("x: {}", x);
}

fn constant() {
    // Constant is always immutable
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);
}

fn shadowing() {
    /*
     * shadowing vs `mut` keyword
     * 1. The variable is still immutable. -> Cannot reassign the new value without `let` keyword.
     * 2. Shadowing can change the variable's type. ex) i32 -> String
     */

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x: {}", x);
}

fn data_types() {
    // let guess = "42".parse().expect("!!");
}

fn main() {
    data_types();
}
