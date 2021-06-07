#![allow(unused)]

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }

    #[test]
    fn test_capturing_env() {
        let x = 4;
        let equal_to_x = |z: i32| z == x;

        let y = 4;
        assert!(equal_to_x(y));
    }

    #[test]
    fn move_capturing() {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }
}
