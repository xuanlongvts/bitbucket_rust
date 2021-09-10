use std::fmt;

fn main() {
	/* ------------ way 1 */
	#[derive(Debug)]
	struct Point1 {
		x: i32,
		y: i32,
	}

	let origin = Point1 {
		x: 1, 
		y: 2
	};
	let str_for = format!("The origin is: {:?}", origin);
	println!("str_for 1: {}", str_for);
	assert_eq!(str_for, "The origin is: Point1 { x: 1, y: 2 }");

	println!("========> origin 1: {:#?}", origin);

	/* ------------ way 2 */
	struct Point2 {
		x: i32,
		y: i32
	}
	impl fmt::Debug for Point2 {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Point2")
				.field("x", &self.x)
				.field("y", &self.y)
				.finish()
		}
	}
	let origin = Point2 {
		x: 2,
		y: 4
	};
	let str_for = format!("The origin is: {:?}", origin);
	println!("str_for 2: {}", str_for);
	assert_eq!(str_for, "The origin is: Point2 { x: 2, y: 4 }");
}