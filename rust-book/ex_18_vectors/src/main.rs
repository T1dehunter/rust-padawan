fn main() {
    println!("Hello, vectors!");

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // let v1 = vec![1, 2, 3];

    test_read();

    test_read_and_add();

    test_iteration();

    test_vectors_with_enum()
}

fn test_read() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    // let ten: &i32 = &v[10];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn test_read_and_add() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    // v.push(6);

    println!("Read the third element is {}", third);
}

fn test_iteration() {
    let mut v = vec![100, 200, 300];

    for i in &mut v {
        *i += 10;
    }

    for i in &v {
        println!("i is: {i}");
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn test_vectors_with_enum() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("vector with spreadsheet cells {:#?}", row);
}
