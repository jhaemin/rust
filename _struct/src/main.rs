struct User {
  email: String,
  username: String,
}

fn main() {
  let user1 = User {
    email: String::from("io@jhaemin.com"),
    username: String::from("jhaemin"),
  };

  let user2 = User {
    email: String::from("io@jhaemin.com"),
    ..user1
  };
}
