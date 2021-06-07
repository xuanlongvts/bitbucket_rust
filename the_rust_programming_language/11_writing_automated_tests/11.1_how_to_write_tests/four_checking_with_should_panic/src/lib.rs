#![allow(unused)]

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    fn new2(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    fn new3(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            )
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            )
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn great_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic]
    fn great_than_100_2() {
        Guess::new2(300);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn great_than_100_3() {
        Guess::new3(300);
    }
}
