struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("user.name:{0},user.email:{1}", user1.username, user1.email);
    // println!("color:{0};point:{1}", black, origin);
}
