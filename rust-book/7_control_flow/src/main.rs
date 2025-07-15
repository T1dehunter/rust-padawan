fn main() {
    println!("Hello, control flow!");

    test_if();

    test_loop();

    test_loop_lables();

    test_while_loop();

    test_for_loop()
}

fn test_if() {
    let x = 5;
    if x < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let n = true;

    if n {
        println!("n exists");
    }

    let cond = true;

    let like_in_python = if cond { "it is like in python" } else { "it is not like" };

    println!("The value of like_in_python is: {like_in_python}");

    // same types in branches
    // let cond1 = true;
    // let number1 = if cond1 { 5 } else { "6" };
}

fn test_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn test_loop_lables() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn test_while_loop() {
    let a = [1, 2, 3];
    let mut index = 0;

    while index < 3 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("index is {index}", index = index);
}

fn test_for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the element is: {}", element);
    }

    for item in a.iter() {
        println!("the item is: {}", item);
    }

    for number in 1..4 {
        println!("the number is: {}", number)
    }
}