#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, using structs!");

    let rect = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
