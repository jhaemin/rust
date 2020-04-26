fn main() {
  // Infinite loop
  // loop {
  //   println!("again!");
  // }

  // loop
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {}", result);

  // while
  let mut number = 3;

  while number != 0 {
    println!("{}!", number);

    number -= 1;
  }

  println!("LIFTOFF!!!");

  let aa = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < 5 {
    println!("the value is: {}", aa[index]);

    index += 1;
  }

  // for
  let a = [10, 20, 30, 40, 50];

  for element in a.iter() {
    println!("the value is: {}", element);
  }

  for number in 1..4 {
    println!("{}!", number);
  }

  for number in (1..4).rev() {
    println!("{}!", number);
  }
}
