use std::collections::HashMap;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
    }
}

use self::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

fn main() {
    println!("Hello, world!");
    let mut map = HashMap::new();
    map.insert(1,2);
}
