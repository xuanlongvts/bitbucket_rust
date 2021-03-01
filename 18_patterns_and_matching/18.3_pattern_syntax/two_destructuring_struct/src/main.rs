struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let Point { x: a, y: b } = p1;
    assert_eq!(10, a);
    assert_eq!(20, b);

    let Point { x, y } = p1;
    assert_eq!(10, x);
    assert_eq!(20, y);

    match p1 {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 10, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: {}, {}", x, y),
    }
}
