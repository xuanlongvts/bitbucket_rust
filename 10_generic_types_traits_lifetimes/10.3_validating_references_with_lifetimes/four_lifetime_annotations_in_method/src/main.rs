#![allow(unused)]

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        10
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("part = {}", i.part);
    println!("level = {}", i.level());

    let ann = String::from("long le");
    println!("announce and part = {}", i.announce_and_return_part(&ann));

    // the stati lifetime
    let sta: &'static str = "I have a static lifetime.";
    println!("sta: = {}", sta);
}
