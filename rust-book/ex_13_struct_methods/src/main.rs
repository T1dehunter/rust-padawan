#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Hello, struct methods!");

    let rect1 = Rectangle {
        width: 100,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    let square = Rectangle::square(30);

    println!(
        "The area of the rect1 is {} square pixels.",
        rect1.area()
    );

    println!("Rect1 can hold rect2? {}", rect1.can_hold(&rect2));

    println!("square {square:?}");
}
