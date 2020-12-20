#![allow(unused)]

fn main() {
    let integer = Some(5);
    let float = Some(10.2);

    // the same
    enum OptionI32 {
        Some(i32),
        None,
    }
    enum OptionF64 {
        Some(f64),
        None,
    }

    let i = OptionI32::Some(10);
    let f = OptionF64::Some(2.2);
}
