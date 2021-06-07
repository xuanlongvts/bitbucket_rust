#![allow(unused)]

fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    };

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.22),
        SpreadsheetCell::Text(String::from("long le")),
    ];
}
