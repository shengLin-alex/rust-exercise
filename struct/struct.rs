struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    let user2 = create_user(user1);

    let user3 = build_user(user2.email);


    // in this case user1 and user2.email is move!
    println!("{}{}", user2.username, user3.email);
}

fn create_user(user: User) -> User {
    let user2 = User {
        active: false,
        sign_in_count: 4,

        // use other user property with same value
        ..user
    };

    user2
}

fn build_user(email: String) -> User {
    User {
        email,
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    }
}