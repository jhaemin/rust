fn main() {
    rev();
}

fn general_if() {
    let number = 7;

    if number < 5 {
        println!("matched");
    } else {
        println!("mismatched");
    }
}

fn if_else_if() {
    let number = 12;

    if number % 4 == 0 {
        println!("number divided by 4");
    } else if number % 3 == 0 {
        println!("number divided by 3");
    } else if number % 2 == 0 {
        println!("number divided by 2");
    } else {
        println!("number doesn't divided by any number");
    }
}

fn if_expression() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number: {}", number);
}

fn loop_() {
    loop {
        println!("inside loop");
    }
}

fn break_() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {}", result);
}

fn while_() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Fire!");
}

fn loop_through_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("value: {}", a[index]);

        index = index + 1;
    }
}

fn loop_through_collection_2() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("value: {}", element);
    }
}

fn rev() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("Fire!");
}
