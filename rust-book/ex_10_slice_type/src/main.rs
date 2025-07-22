fn main() {
    println!("Hello, slice type!");

    let mut text = String::from("hello world, how are you?");
    let len = text.len();
    let hello = &text[0..5];
    let world = &text[6..11];
    let rest = &text[6..];
    let copy = &text[..len];
    let copy1 = &text[..];

    println!("test slices: {hello} {world}");

    println!("rest: {rest}");

    println!("copy: {copy}");

    println!("copy1: {copy1}");

    let word = first_word(&text);

    // text.clear();

    println!("first word: {word}");

    // println!("the text is: {text}");
}


fn first_word(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }

    &str[..]
}