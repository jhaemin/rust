use std::io;

fn main() {
  let mut n = String::new();

  println!("n: ");

  io::stdin().read_line(&mut n).expect("Failed to read step");

  let n: i32 = match n.trim().parse() {
    Ok(num) => num,
    Err(_) => 0,
  };

  if n == 0 {
    println!("Fibonacci: 0");
    return;
  } else if n == 1 {
    println!("Fibonacci: 1");
    return;
  } else if n < 0 {
    println!("Error");
    return;
  }

  let mut prev = 0;
  let mut next = 1;
  let mut sum = prev + next;

  for _ in 2..n {
    prev = next;
    next = sum;
    sum = prev + next;
    println!("prev: {}, next: {}, sum: {}", prev, next, sum);
  }

  println!("{}", sum);
}
