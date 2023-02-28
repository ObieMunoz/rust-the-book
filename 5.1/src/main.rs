// Struct
struct User { 
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // Tuple struct
struct Point(i32, i32, i32); // Tuple struct

struct AlwaysEqual; // unit-like struct

fn main() {
    let mut user1 = User {
        email: String::from("obiemunozjr@gmail.com"),
        username: String::from("obiemunoz"),
        active: true,
        sign_in_count: 1,
    };

    println!("User 1: {}", user1.email);

    user1.email = String::from("obie@gmail.com");

    println!("User 1: {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("User 2: {}", user2.email);

    let user3 = build_user(String::from("test3@gmail.com"), String::from("test3"));

    println!("User 3: {}", user3.email);

    let black = Color(0,0,0);

    println!("Black: {}, {}, {}", black.0, black.1, black.2);

    let origin = Point(0,0,0);

    println!("Origin: {}, {}, {}", origin.0, origin.1, origin.2);

    let subject = AlwaysEqual;
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
