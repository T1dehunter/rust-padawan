use std::io;

fn main() {
    println!("Hello, data types!");

    let guess: i32 = "42".parse().expect("Not a number!");

    println!("The value of guess is: {guess}");

    // let mut test_over: u8 = 255;
    let mut test_over: u8 = 254;

    test_over += 1;
    println!("The value of test is: {test_over}");

    test_char();

    test_tuple();

    test_arr();

    test_vec();

    // test_arr_invalid_access()
}

fn test_char() {
    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");
}

fn test_tuple() {
    let mut tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let test = tup.0;

    println!("The value of test is: {test}");

    tup.1 = 124.5;

    let (_, new, _) = tup;

    println!("Test y after change: {y}");
    println!("Test y after change: {new}");
}

fn test_arr() {
    let my_arr = [1, 2, 3, 4, 5, 6, 7, 8];

    let first = my_arr[0];
    println!("The value of first is: {first}");

    let last = my_arr.last().unwrap();
    println!("The value of last is: {last}");

    // let my_arr1: [i32; 5] = [1, 2, 3, 4, 5];
    //
    // let my_arr2 = [9; 5];


}

fn test_vec() {
    let my_arr_vec: Vec<char> = vec!['H', 'E', 'L', 'L', 'O', '-', 'W', 'O', 'R', 'L', 'D'];
    let first = my_arr_vec[0];
    println!("The value of vec first is: {first}");
}

fn test_arr_invalid_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let mut element: i32 = 0;
    if let Some(value) = a.get(index) {
        element = *value;
        println!("The value of the array is: {element}");
    } else {
        println!("The value of the array was not found");
    }

    println!("The value of the element at index {index} is: {element}");
}