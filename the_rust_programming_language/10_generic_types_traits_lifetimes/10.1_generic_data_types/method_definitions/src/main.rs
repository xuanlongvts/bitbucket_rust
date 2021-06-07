#![allow(unused)]

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x_value(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2).sqrt()
    }
}

struct Points<T, U> {
    x: T,
    y: U,
}

impl<T, U> Points<T, U> {
    fn mixup<V, W>(self, other: Points<V, W>) -> Points<T, W> {
        Points {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x_value());

    let p1 = Points { x: 5, y: 10.5 };
    let p2 = Points { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
