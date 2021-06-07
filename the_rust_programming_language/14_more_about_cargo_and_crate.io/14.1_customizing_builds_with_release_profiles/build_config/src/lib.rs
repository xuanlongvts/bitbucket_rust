//! # My crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculation more convenient.
//!
//! # Art
//!
//! A library for modeling artistic concepts.

/// Add one to the number given.
///
/// # Example
///
/// ```
///  let arg = 5;
///  let answer = my_crate::add_one(arg: i32);
///
///  assert_eq!(6, answer);
/// ```
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
