#[derive(Debug)]
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

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn main() {
    println!("Hello, Concise Control Flow!");

    let config_max = Some(5u8);

    // before
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }

    // after
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }


    let coin = Coin::Penny;
    let mut count = 0;

    // before
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => count += 1,
    // }

    // after
    if let Coin::Quarter(state) = coin {
        println!("Quarter from {:?}", state);
    } else {
        count += 1;
    }

    println!("The value of count is: {count}");


    let res = describe_state_quarter(Coin::Quarter(UsState::Alabama));
    println!("Describe state res {:?}", res);
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    println!("Quarter from {:?}", coin);
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
