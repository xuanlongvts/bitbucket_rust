#![allow(unused)]
#[derive(Debug)]

enum UsState1 {
    Alabama,
    Alaska,
}

enum Coin1 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState1)
}

fn main() {
    let coin1 = Coin1::Penny;
    let mut count1 = 0;

    match coin1 {
        Coin1::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count1 += 1
    }

    // let mut count2 = 0;
    // if let Coin1::Quarter(state) = coin1  {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count2 += 1;
    // }
}
