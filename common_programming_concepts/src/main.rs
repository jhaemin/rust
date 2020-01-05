fn main() {
    // Variables and Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // Shadowing
    let a = 5;
    let a = a + 1;
    let a = a * 2;
    println!("The value of a is: {}", a);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);
}
