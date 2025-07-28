#[derive(Debug)]
enum IpAddrType {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrTypeCall {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrType,
    address: String,
}

fn main() {
    println!("Hello, enums!");

    let four = IpAddrType::V4;
    let six = IpAddrType::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrType::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrType::V6,
        address: String::from("::1"),
    };

    println!("Home IP address: {}", home.address);
    println!("Loopback IP address: {}", loopback.address);

    let home1 = IpAddrTypeCall::V4(127, 0, 0, 1);
    let loopback1 = IpAddrTypeCall::V6(String::from("::1"));

    println!("home1 is: {:#?}", home1);
    println!("loopback1 is: {:#?}", loopback1);

    test_enum_like_struct();

    test_some_option()
}

fn route(ip: IpAddrType) {
    println!("IP is: {:#?}", ip)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn test_enum_like_struct() {
    let msg1 = Message::Write(String::from("hello msg!"));
    let msg2 = Message::Move { x: 10, y: 10 };
    let msg3 = Message::Quit;
    let msg4 = Message::ChangeColor(1, 2, 3);

    test_handle_message(msg1);
    test_handle_message(msg2);
    test_handle_message(msg3);
    test_handle_message(msg4);
}

fn test_handle_message(msg: Message) {
    println!("received msg: {:#?}", msg);
}

fn test_some_option() {
    let some_number_1 = Some(5);
    let some_number_2 = Some(7);

    // doesnt works like in TS :)
    // if some_number_1.is_some() && some_number_2.is_some() {
    //     let sum = some_number_1 + some_number_2;
    // }
    if let (Some(n1), Some(n2)) = (some_number_1, some_number_2) {
        let sum = n1 + n2;
        println!("Options sum is {}", sum);
    } else {
        println!("One of the options is None");
    }
}


