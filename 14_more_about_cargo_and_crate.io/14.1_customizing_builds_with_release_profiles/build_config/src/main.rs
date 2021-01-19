use build_config::add_one;
// use build_config::kinds::PrimaryColor;
// use build_config::utils::mix;

use build_config::mix;
use build_config::PrimaryColor;

fn main() {
    let total: i32 = add_one(10);
    println!("total = {}", total);

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    mix(red, yellow);
}
