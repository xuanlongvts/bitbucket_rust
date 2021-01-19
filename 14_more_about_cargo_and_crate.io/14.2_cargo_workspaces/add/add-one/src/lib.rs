use rand;


pub fn random_number() -> i32 {
    let ran = rand::random();
    ran
}
 
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        assert_eq!(2, add_one(1));
    }
}
