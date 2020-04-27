fn main() {
  let s1 = String::from("hello");
  let s2 = s1;

  // println!("{}, world!", s1); // Compile error (s1 is invalid)
  println!("{}, world!", s2);

  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2);

  let s = String::from("Takes Ownership");

  takes_ownership(s);
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
}
