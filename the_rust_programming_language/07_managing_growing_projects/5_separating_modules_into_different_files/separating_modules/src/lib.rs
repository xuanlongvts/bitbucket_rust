mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;

// self :: file  :: pub mod
pub use self::back_of_house::back_of_house::hosting_back;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    hosting_back::yard_fruit();
}
