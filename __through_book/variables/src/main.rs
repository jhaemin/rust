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

fn calc() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
}

fn boolean() {
    let t = true;
    let f: bool = false;
}

fn character() {
    let c = 'z';
    let z = 'z';
    let heart_eyed_cat = '😻';
}

fn compound_types() {
    /*
     * Tuples
     */
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    // Destructuring
    let (x, y, z) = tup;
    println!("y: {}", y);

    let five_hundred = tup.0;
    println!("five_hundred: {}", five_hundred);
    let six_point_four = tup.1;
    println!("six_point_four: {}", six_point_four);
    let one = tup.2;
    println!("one: {}", one);

    /*
     * Arrays
     */
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first = a[0];
    let second = a[1];
    println!("first: {}", &first);
    println!("second: {}", &second);
}

fn main() {
    compound_types();
}
