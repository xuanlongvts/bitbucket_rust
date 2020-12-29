#![allow(unused)]

pub fn greeting(name: &str) -> String {
    format!("hello {}!", name)
}

pub fn greeting2(name: &str) -> String {
    String::from("Hello 222")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name2() {
        let result = greeting2("long le");
        // assert!(result.contains("long le"));
        assert!(
            result.contains("long le"),
            "Greeting did not contain name, value was `{}`",
            result
        )
    }
}
