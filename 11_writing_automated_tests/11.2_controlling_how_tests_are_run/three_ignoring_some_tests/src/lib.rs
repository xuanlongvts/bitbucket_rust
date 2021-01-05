#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(4 - 2, 2);
    }
}

// cargo test -- --ignored for ignored function expensive_test
