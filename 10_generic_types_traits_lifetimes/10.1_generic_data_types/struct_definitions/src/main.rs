#![allow(unused)]

struct Point<T> {
    x: T,
    y: T,
}

struct Points<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 2.2, y: 5.5 };

    // let err_differ_type = Point { x: 2, y: 2.5 }; // error because x and y different type
    let different_typs = Points { x: 1, y: 10.2 };
}
