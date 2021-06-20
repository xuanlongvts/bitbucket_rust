use std::num::ParseIntError;

// Define a generic alias for a `Result` with the error type `ParseIntError`.
type AliasedResult<T> = Result<T, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
fn multiply(firs: &str, second: &str) -> AliasedResult<i32> {
	firs.parse::<i32>().and_then(|fir_num| {
		second.parse::<i32>().map(|sec_num| fir_num * sec_num)
	})
}

fn print(num: AliasedResult<i32>) {
	match num {
		Ok(v) => println!("number: {}", v),
		Err(e) => println!("error: {}", e),
	}
}

fn main() {
	let twe = multiply("10", "2");
	print(twe);

	let twe_v2 = multiply("tt", "2");
	print(twe_v2);
}