pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test_1() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn add_test_2() {
        assert_ne!(add(2, 2), 5);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(bad_add(2, 2), 4);
    }
}
