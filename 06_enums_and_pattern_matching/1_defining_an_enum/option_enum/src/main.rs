#![allow(unused)]

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let x: i8 = 5;
    // let y: Option<i8> = Some(5);
}
