#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("Hello, world!");
    println!("IP address: {:?}", four);

    println!("A quarter is {} cents.", value_in_cents(Coin::Quarter));

    match some_number {
        None => (),
        Some(i) => {
            println!("The number plus one is: {}", i + 1);
        }
    };

    match absent_number {
        _ => println!("Catch all!"),
    }

    if let Some(string) = some_string {
        println!("Some string is: {}", string);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
