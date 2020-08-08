fn main() {
  // `{}` will be automatically replaced with any arguments
  println!("{} days", 31);

  // Positional arguments
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  // Named arguments
  println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");

  // Special formatting can be specified after a `:`
  // `:b` formats integer to binary. ex) 2 -> 10, 3 -> 11
  println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
}