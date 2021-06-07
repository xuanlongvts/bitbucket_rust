#![allow(unused)]

use add_one;
use rand;

fn main() {
    let num = 10;

    println!("Hello, {} plus one = {}", num, add_one::add_one(num));
}
