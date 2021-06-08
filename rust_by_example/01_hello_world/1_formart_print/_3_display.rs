use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);
// Implement `Display` for MinMax
impl fmt::Display for MinMax {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}, {}", self.0, self.1)
	}
}

#[derive(Debug)]
struct Point2D {
	x: f64,
	y: f64,
}
// Implement `Display` for `Point2D`
impl fmt::Display for Point2D {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "x: {}, y: {}", self.x, self.y)
	}
}

fn main() {
	let min_max = MinMax(0, 14);

	println!("Compare structures:");
    println!("Display: {}", min_max);
    println!("Debug: {:?}", min_max);

	let point2d = Point2D {
		x: 1.2,
		y: 3.4,
	};
	println!("Compare point2d:");
    println!("Display: {}", point2d);
    println!("Debug: {:?}", point2d);
}