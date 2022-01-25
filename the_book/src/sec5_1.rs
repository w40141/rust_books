struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };

    let mut user2 = User {
        email: String::from("sometwo@example.com"),
        username: String::from("sometwoname123"),
        sign_in_count: 2,
        active: true,
    };

    user2.email = String::from("anotheremail@example.com");

    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
