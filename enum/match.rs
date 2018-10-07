#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_coin(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);

            25
        },
    }
}

fn match_option(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(i) => Some(i + 10)
    }
}

fn default_match() {
    let value = 2u8;
    match value {
        1 => println!("one"),
        2 => println!("two"),

        // use _
        _ => println!("default")
    }
}

fn not_if_let() {
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);

    match coin {
        Coin::Quarter(state) => println!("{:?}", state),
        _ => count += 1
    }
}

fn use_if_let() {
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);

    // 使用 if let 來達到 not_if_let() 的行為
    if let Coin::Quarter(state) = coin {
        println!("{:?}", state);
    } else {
        count += 1;
    }
}

fn main() {

    let coin = Coin::Quarter(UsState::Alaska);

    println!("{}", value_in_coin(&coin));

    let five = Some(5);
    let fifteen = match_option(five);

    println!("{:?}", fifteen);

    default_match();
}