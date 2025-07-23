
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    has_lightsaber: bool,
}

fn main() {
    println!("Hello, structs!");

    let mut user1 = build_user(String::from("Luke_Skaywalker"), String::from("luke_skaywalker@mail.com"));
    user1.has_lightsaber = false;
    println!("User1: {user1:?}");

    let user2 = test_build_user(user1, String::from("Luke_I_am_your_father"), String::from("darth_vaider@mail.com"));
    println!("User2: {user2:?}");

    test_tuple_structs();

    let empty_struct = test_empty_struct();
    println!("Empty: {empty_struct:?}");
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        has_lightsaber: true,
    }
}

fn test_build_user(user: User, username: String, email: String) -> User {
    User {
        username,
        email,
        ..user
    }
}

#[derive(Debug)]
struct Empty;
fn test_empty_struct() -> Empty {
    let empty = Empty;
    empty
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn test_tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("color black: ({}, {}, {})", black.0, black.1, black.2);

    println!("point origin: ({}, {}, {})", origin.0, origin.1, origin.2);
}

