use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, error handling!");
    // panic!("crash and burn");

    test_read_file();
    test_read_with_expect();
    let _ = read_username_from_file();
    let r = read_username_from_file_upd_1();
    println!("res {:?}", r);

    let r = read_username_from_file_upd_2();
    println!("res {:?}", r);
}

fn test_read_file() {
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        // let line = line; // Handle potential errors for each line
        println!("{}", line);
    }
    // println!("File content:\n{}", text_from_file.t);
}

fn test_read_with_expect() {
    let _file = File::open("hello.txt").expect("hello.txt should be included in this project");
    // let _file = File::open("hello.txtt").unwrap();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_upd_1() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txtt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_upd_2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
