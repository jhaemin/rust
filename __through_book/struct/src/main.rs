struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        username: String::from("jhaemin"),
        email: String::from("io@jhaemin.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("email: {}", user1.email);
    user1.email = String::from("fameu5e@cau.ac.kr");
    println!("email: {}", user1.email);

    let user2 = build_user(String::from("fameu5e@naver.com"), String::from("fameu5e"));
    let user3 = User {
        email: String::from("jhaemin@kakao.com"),
        username: String::from("jhaemin_kakao"),
        active: user2.active,
        sign_in_count: user2.sign_in_count,
    };

    let user4 = User {
        email: String::from("io@jhaemin.com"),
        username: String::from("jhaemin"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
