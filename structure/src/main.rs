struct User {
    user: String,
    email: String,
    is_active: bool,
}

fn main() {
    let user_info = set_user(
        String::from("ehdgus1524@gmail.com"),
        String::from("donghquinn"),
    );

    // println!("User Info: {:#?}", user_info);
    println!("User Email: {}", user_info.email);
}

fn set_user(email: String, user: String) -> User {
    User {
        user,
        email,
        is_active: true,
    }
}
