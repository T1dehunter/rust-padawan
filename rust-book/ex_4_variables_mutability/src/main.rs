fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let v = get_value();
    println!("The value of v is: {v}");

    test_const();

    test_shadowing();

    test_reuse();
}

fn test_const() {
    const MY_CONST: u32 = 10_000;
    // const MY_CONST2: u32 = get_value();
    println!("The const value of MY_CONST is: {MY_CONST}");
}

fn get_value() -> u32 {
    100
}

fn test_shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");
}

fn test_reuse() {
    let spaces = "   ";

    let mut spaces = spaces.len();

    spaces += 1;

    println!("The value of spaces is: {spaces}");
}
