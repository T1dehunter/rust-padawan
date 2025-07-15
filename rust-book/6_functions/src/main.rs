fn main() {
    println!("Hello, 6_functions!");

    another_function(1, 'Y');

    test_expression()
}

fn another_function(x: i32, y: char) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn test_expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}