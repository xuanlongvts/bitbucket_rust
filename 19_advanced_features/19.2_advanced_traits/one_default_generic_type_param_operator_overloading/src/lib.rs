use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }

        // for test faile
        // Point {
        //     x: self.x - other.x,
        //     y: self.y - other.y,
        // }
    }
}

#[derive(Debug, PartialEq)]
struct Milimeters(u32);
struct Meters(u32);

impl Add<Meters> for Milimeters {
    type Output = Milimeters;
    fn add(self, other: Meters) -> Milimeters {
        Milimeters(self.0 + (other.0 * 5))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        )
    }

    #[test]
    fn milimeters_test() {
        assert_eq!(Milimeters(10), Milimeters(10))
    }
}
