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

  // Numeric Operations
  let sum = 5 + 10;
  let difference = 95.5 - 4.3;
  let product = 4 * 30;
  let quotient = 56.7 / 32.2;
  let remainder = 43 % 5;
  println!(
    "{} {} {} {} {}",
    sum, difference, product, quotient, remainder
  );

  // Compound Types
  // Tuples
  let tup: (i32, f64, u8) = (500, 6.4, 1);

  // Tuples - Pattern Matching(Destructuing)
  let (x, y, z) = tup;
  println!("x: {},  y: {}, z: {}", x, y, z);

  // Tuples - Access with index
  println!("{} {} {}", tup.0, tup.1, tup.2);

  // Arrays
  let _months = [
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
  let _array: [i32; 5] = [1, 2, 3, 4, 5];
  let _array_init = [3; 5];

  // Using functions
  another_function(5);
  println!("Result of five(): {}", five());
  println!("99 + 1 = {}", plus_one(99));
}

fn another_function(x: i32) {
  println!("The value of x is: {}", x);
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}
