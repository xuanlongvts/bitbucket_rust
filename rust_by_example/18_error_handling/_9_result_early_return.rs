use std::num::ParseIntError;

type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first_num: &str, second_num: &str) -> AliasedResult<i32> {
	let first = match first_num.parse::<i32>() {
		Ok(v) => v,
		Err(e) => return Err(e),
	};

	let second = match second_num.parse::<i32>() {
		Ok(v) => v,
		Err(e) => return Err(e)
	};
	Ok(first * second)
}

fn print(result: AliasedResult<i32>) {
	match result {
		Ok(n) => println!("Result: {}", n),
		Err(e) => println!("Error: {}", e),
	}
}

fn main() {
	let twe_1 = multiply("10", "2");
	print(twe_1);

	let twe_2 = multiply("tt", "2");
	print(twe_2);
}