#![allow(unused)]

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result1 = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result1);

    println!("============================");
    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result2 = longest(string3.as_str(), string4.as_str());
        println!("The longest string is: {}", result2);
    }

    println!("============================");
    let string5 = String::from("String 5");
    let result3;
    {
        let string6 = String::from("xyz");
        result3 = longest2(string5.as_str(), string6.as_str());
    }
    println!("result3: {}", result3);
}

/* Error
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Error, because returns a value referencing data owned by the current function
/*
fn longest3<'a>(x: &'a str, y: &str) -> &'a str {
    let result = String::from("realy long string");
    result.as_str();
}
*/
