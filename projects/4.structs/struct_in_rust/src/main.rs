struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}



fn main() {
    let user1 = User {
        active: true,
        username,
        email,
        sing_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("maria@osteria.com"),
        sign_in_count: user1.sign_in_count,
    };
}
