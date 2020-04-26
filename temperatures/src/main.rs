use std::io;

fn main() {
  loop {
    println!("Input celsius");

    let mut celsius = String::new();

    io::stdin()
      .read_line(&mut celsius)
      .expect("Failed to read celsius");

    let celsius: u64 = match celsius.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    let fahrenheit: u64 = (celsius * 9 / 5) + 32;

    println!("Fahrenheit: {}", fahrenheit);
  }
}
