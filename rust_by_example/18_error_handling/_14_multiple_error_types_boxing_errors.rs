use std::error;
use std::fmt;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "invalid first item to double")
	}
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
	// ok_or_else Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err()).
	/*
		let x = Some("foo");
		assert_eq!(x.ok_or_else(|| 0), Ok("foo"));

		let x: Option<&str> = None;
		assert_eq!(x.ok_or_else(|| 0), Err(0));
	*/
	vec.first().ok_or_else(|| EmptyVec.into()) // Converts to Box
		.and_then(|s| {
			s.parse::<i32>()
			.map_err(|e| e.into()) // Converts to Box
			.map(|v| v * 2)
		})
}

fn print(result: Result<i32>) {
	match result {
		Ok(v) => println!("The first doubled is {}", v),
		Err(e) => println!("Error: {}", e),
	}
}

fn main() {
	let numbers = vec!["5", "10", "20"];
	let empty = vec![];
	let strings = vec!["tt", "1", "2"];

	print(double_first(numbers));
	print(double_first(empty));
	print(double_first(strings));
}