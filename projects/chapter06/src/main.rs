#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

// Listing 6-2 (allow dead code gets rid of the cargo warning)
#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Washington
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    StateQuarter(UsState),
    HalfDollar,
    SacagaweaDollar,
    SusanBAnthony,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::StateQuarter(state) => {
            println!("\tStateQuarter from {:?}", state);
            25
        }
        Coin::HalfDollar => 50,
        Coin::SacagaweaDollar => 100,
        Coin::SusanBAnthony => 100,
    }
}

fn main() {
    let v4_loopback = IpAddrKind::V4(String::from("127.0.0.1"));
    let v6_loopback = IpAddrKind::V6(String::from("::1"));
    println!("v4_loopback: {:?}, v6_loopback: {:?}", v4_loopback, v6_loopback);

    let _some_number = Some(42);
    let _some_string = Some("fortytwo");
    // This seems a little long winded.  Why couldn't I just None<i32>?
    let _absent_number: Option<i32> = None;

    println!("value_in_cents({:?})=>{}", Coin::Penny, value_in_cents(&Coin::Penny));
    println!("value_in_cents({:?})=>{}", Coin::Nickel, value_in_cents(&Coin::Nickel));
    println!("value_in_cents({:?})=>{}", Coin::Dime, value_in_cents(&Coin::Dime));
    println!("value_in_cents({:?})=>{}", Coin::Quarter, value_in_cents(&Coin::Quarter));
    println!(
        "value_in_cents({:?})=>{}",
        Coin::StateQuarter(UsState::Alabama),
        value_in_cents(&Coin::StateQuarter(UsState::Alabama)));
    println!(
        "value_in_cents({:?})=>{}",
        Coin::StateQuarter(UsState::Alaska),
        value_in_cents(&Coin::StateQuarter(UsState::Alaska)));
    println!(
        "value_in_cents({:?})=>{}",
        Coin::StateQuarter(UsState::Washington),
        value_in_cents(&Coin::StateQuarter(UsState::Washington)));
    println!("value_in_cents({:?})=>{}", Coin::HalfDollar, value_in_cents(&Coin::HalfDollar));
    println!(
        "value_in_cents({:?})=>{}",
        Coin::SacagaweaDollar,
        value_in_cents(&Coin::SacagaweaDollar));
    println!(
        "value_in_cents({:?})=>{}",
        Coin::SusanBAnthony,
        value_in_cents(&Coin::SusanBAnthony));

    let _five = Some(5);
    let _six = plus_one(_five);
    let _none = plus_one(None);

    let zero_u8_value = Some(0u8);
    if let Some(3) = zero_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
    let three_u8_value = Some(3u8);
    if let Some(3) = three_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
