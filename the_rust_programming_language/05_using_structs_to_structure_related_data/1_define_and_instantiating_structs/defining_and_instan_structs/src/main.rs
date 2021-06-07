struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("some_user_123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another_email@example.com");
    user1.username = String::from("another_email");

    let user2 = User {
        email: String::from("someone_2@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!("user 2 email: {}", user2.email);

    let user3 = User {
        email: String::from("someone_3@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("user 3 email: {}", user3.email);

    let new_build_user = build_user(String::from("longlx@gmail.com"), String::from("longlx"));
    println!("new_build_user = {}", new_build_user.email);
}
