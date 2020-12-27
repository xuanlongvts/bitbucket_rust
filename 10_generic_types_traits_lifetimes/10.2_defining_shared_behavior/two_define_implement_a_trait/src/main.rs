#![allow(unused)]

use std::fmt::Debug;
use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// impl Summary for NewsArticle {}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// the sam
pub fn notify_the_same<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/* Trait Bound Syntax */
pub fn notify1(item1: &impl Summary, item2: &impl Summary) {}

pub fn notify2<T: Summary>(item: &T) {}

pub fn notify3<T: Summary>(item1: &T, item2: &T) {}

/* Specifying Multiple Trait Bounds with the + Syntax */
pub fn notify4(item: &(impl Summary + Display)) {}
// the same
pub fn notify5<T: Summary + Display>(item: &T) {}

/* Clearer Trait Bounds with where Clauses */
pub fn notify6<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
// the same
pub fn notify7<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

/* Returning Types that Implement Traits */
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
