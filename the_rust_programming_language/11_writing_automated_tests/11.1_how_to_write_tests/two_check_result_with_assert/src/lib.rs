#![allow(unused)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_three(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger1 = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller1 = Rectangle {
            width: 5,
            height: 2,
        };

        assert!(larger1.can_hold(&smaller1));
    }

    #[test]
    fn larger_can_hold_smaller2() {
        let larger2 = Rectangle {
            width: 5,
            height: 6,
        };
        let smaller2 = Rectangle {
            width: 7,
            height: 8,
        };
        assert!(larger2.can_hold(&smaller2));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn it_adds_three() {
        assert_ne!(4, add_three(6))
    }
}
