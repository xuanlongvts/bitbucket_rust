use std::error;
use std::error::Error as _;
use std::num::ParseIntError;
use std::fmt;


#[derive(Debug)]
enum DoubleError {
	EmptyVec,

	// We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
	Parse(ParseIntError)
}

type Result<T> = std::result::Result<T, DoubleError>;

impl fmt::Display for DoubleError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
			DoubleError::Parse(..) => write!(f, "the provided string could not be parsed as int"),
		}
	}
}

impl error::Error for DoubleError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match *self {
			DoubleError::EmptyVec => None,
			DoubleError::Parse(ref e) => Some(e)
		}
	}
}

// Implement the conversion from `ParseIntError` to `DoubleError`.
// This will be automatically called by `?` if a `ParseIntError`
// needs to be converted into a `DoubleError`.
impl From<ParseIntError> for DoubleError {
	fn from(err: ParseIntError) -> DoubleError {
		DoubleError::Parse(err)
	}
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
	let first = vec.first().ok_or(DoubleError::EmptyVec)?;
	let parsed = first.parse::<i32>()?;

	Ok(parsed * 2)
}

fn print(result: Result<i32>) {
	match result {
		Ok(v) => println!("The first doubled is {}", v),
		Err(e) => {
			println!("Error: {}", e);
			if let Some(source) = e.source() {
				println!(" Caused by: {}", source);
			}
		}
	}
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}