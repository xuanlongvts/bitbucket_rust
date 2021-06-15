use rand::prelude::*;

use bar::bar;

fn main() {
    println!("Hello, world! This is FOO");

    let x: u8 = random();
    println!("x = {}", x);

    bar::hi_bar();
}
