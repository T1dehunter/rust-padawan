
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
}

fn take_ownership(some_string: String) {
    println!("take_ownership {}", some_string);
}