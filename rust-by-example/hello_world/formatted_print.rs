fn main() {
    // `{}` will be automatically replaced with any arguments
    println!("{} days", 31);

    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Special formatting can be specified after a `:`
    // `:b` formats integer to binary. ex) 2 -> 10, 3 -> 11
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // Right-align text with a specified width
    println!("{number:>width$}", number = 1, width = 6);

    // With extra zeroes
    println!("{number:>0width$}", number = 1, width = 6);

    // Rust checks to make sure the correct number of arguments are used
    // println!("My name is {0}, {1} {0}", "Bond"); // Wrong
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Create a structure named `Structure` which contains an `i32`
    #[allow(dead_code)]
    struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3)); // Error

    // Print estimated Pi
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
    println!("Pi is roughly {:.precision$}", pi, precision = 3);
}
