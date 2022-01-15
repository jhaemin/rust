fn main() {
    // another_function(5, 6);
    // expressions();

    // let x = five();
    // println!("x: {}", x);

    let x = plus_one(5);
    println!("x: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("x: {}", x);
    println!("y: {}", y);
}

fn expressions() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("y: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
