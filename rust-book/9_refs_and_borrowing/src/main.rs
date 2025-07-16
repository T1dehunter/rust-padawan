fn main() {
    println!("Hello, refs and borrowing!");

    let str = String::from("hello");

    let len = calculate_length(&str);

    println!("The length of '{str}' is {len}.");

    let mut mut_str = String::from("hello hello hello");

    change_str(&mut mut_str);

    println!("changed str: '{mut_str}'");

    test_multi_borrow()
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn change_str(some_string: &mut String) {
    some_string.push_str(", world");
}

fn test_multi_borrow() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    // let r3 = &mut s;

    println!("test_multi_borrow {r1}, {r2}, and {}", r3);
}