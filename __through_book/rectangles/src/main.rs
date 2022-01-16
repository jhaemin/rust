fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("{}", area(width1, height1));

    // Using tuples
    let rect1 = (30, 50);

    println!("{}", area2(rect1));

    // Using structs
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", area3(&rect2));

    println!("{:#?}", rect2);

    println!("{}", rect2.area());

    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("can rect2 hold rect4? {}", rect2.can_hold(&rect4));

    let rect4 = Rectangle::square(50);

    println!("{:#?}", rect4);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
