enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    println!("Hello, match control flow!");

    test_match_enum();

    test_match_and_increment();

    test_match_other();
}

fn test_match_enum() {
    println!("Penny value in cents is {}", value_in_cents(Coin::Penny));
    println!("Nickel value in cents is {}", value_in_cents(Coin::Nickel));
    println!("Dime value in cents is {}", value_in_cents(Coin::Dime));
    println!("Quarter value in cents is {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn test_match_and_increment() {
    let five = Some(5);

    let six = plus_one(five);
    println!("plus_one five is {:?}", six);

    let none = plus_one(None);
    println!("plus_one none is {:?}", none);
}

fn plus_one(number: Option<i32>) -> Option<i32> {
    match number {
        None => None,
        Some(value) => Some(value + 1),
    }
}

fn test_match_other() {
    let dice_roll = 8;
    match dice_roll {
        3 => println!("Dice rolled 3 times"),
        7 => println!("Dice rolled 7 times once"),
        other => println!("Dice rolled other than 3,7 => {}", other),
    }
}