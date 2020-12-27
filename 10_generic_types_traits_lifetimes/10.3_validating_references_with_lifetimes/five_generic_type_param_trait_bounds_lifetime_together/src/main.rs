use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let string3 = "Today is someone's birthday!";
    let result1 = longest_with_an_announcement(string1.as_str(), string2, string3);
    println!("The longest string is: {}", result1);
}
