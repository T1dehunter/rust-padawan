fn main() {
    println!("Hello, strings!");

    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    let s_ukr = "Привіт мир!".to_string();

    println!("str: {}", s);

    println!("s_ukr: {}", s_ukr);

    test_str_indexing();

    test_str_iter();
}

fn test_str_indexing() {
    let str = String::from("Hello world!");
    // Rust strings don’t support indexing!!!
    // let symbol = str[0];

    let first = &str[0..1];
    println!("first in en str: {}", first);

    let str = String::from("Привіт мир!");
    // this would panic cause all data in str in utf-8 and we cant just get [0] byte!
    // let first = &str[0..1];
    let first = &str[0..2];
    println!("first in ukr str: {}", first);
}

fn test_str_iter() {
    let str = String::from("Hello world!");
    for c in str.chars() {
        println!("en symbol {c}");
    }

    let str = String::from("Привіт мир!");
    for c in str.chars() {
        println!("ukr symbol {c}");
    }
}
