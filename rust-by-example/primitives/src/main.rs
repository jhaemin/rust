fn main() {
  let logical: bool = true;

  let a_float: f64 = 1.0;
  let an_integer = 5i32;

  let default_float = 3.0;
  let default_integer = 7;

  // Inferred from context
  let mut inferred_type = 12;
  inferred_type = 4294967296i64; // Inferred as i64

  let mut mutable = 12;
  mutable = 21;

  // Error! Tye type of a variable can't be changed
  // mutable = true;

  // Shadowing
  let mutable = true;

  // Literals and operators

  // Integer addition
  println!("1 + 2 = {}", 1u32 + 2);

  // Integer subtraction
  println!("1 - 2 = {}", 1i32 - 2);

  // Short-circuiting boolean logic
  println!("true AND false is {}", true && false);
  println!("true OR false is {}", true || false);
  println!("NOT true is {}", !true);

  // Bitwise operations
  println!("0011 AND 0101 is {:04b}", 0b0011 & 0b0101);
  println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
}
