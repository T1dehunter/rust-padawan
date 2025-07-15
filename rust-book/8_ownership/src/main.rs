use std::ops::Add;

fn main() {
    println!("Hello, 8_ownership!");

    simple_str();

    simple_str_from();

    gives_ownership();
}

fn simple_str() {
    let x = "hello world";
    let y = x;

    println!("{x} : {y}");
}

fn simple_str_from() {
    let x = String::from("hello world");
    let y = x;

    // wont work cause x is "moved" to y
    // println!("{x} : {y}");

    println!("yyy: {y}");

    let one = String::from("hello world");
    let two = one.clone();

    println!("one: {one} --- two: {two}");
}

fn gives_ownership() {
    let some_string = String::from("hello world");
    take_ownership(some_string);
    // wont work cause x is "moved" to take_ownership
    // println!("some_string: {some_string}");

    let next_str = String::from("Luke, i am your father!");
    println!("next_str {}", next_str);


    let res = takes_and_gives_back(next_str);
    println!("res {}", res);

    let test = String::from("hello world");
    let (test2, length) = calculate_length(test);

    println!("test {} len {}", test2, length);
}

fn take_ownership(some_string: String) {
    println!("take_ownership {}", some_string);
}

fn takes_and_gives_back(s: String) -> String {
    let some_string = String::from("my text");

    let next = s.add(some_string.as_str());

    next
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}