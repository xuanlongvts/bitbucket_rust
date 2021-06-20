use std::num::ParseIntError;

// With the return type rewritten, we use pattern matching without `unwrap()`.
fn multiply_v1(first_num: &str, second_num: &str) -> Result<i32, ParseIntError> {
	match first_num.parse::<i32>() {
		Ok(firt_n) => {
			match second_num.parse::<i32>() {
				Ok(sec_n) => {
					Ok(firt_n * sec_n)
				},
				Err(e) => Err(e),
			}
		},
		Err(e) => Err(e),
	}
}

// As with `Option`, we can use combinators such as `map()`.
// This function is otherwise identical to the one above and reads:
// Modify n if the value is valid, otherwise pass on the error.
fn multiply_v2(first_num: &str, second_num: &str) -> Result<i32, ParseIntError> {
	first_num.parse::<i32>().and_then(|first_n| {
		second_num.parse::<i32>().map(|second_n| first_n * second_n)
	})
}

fn print(result: Result<i32, ParseIntError>) {
	match result {
		Ok(v) => println!("value is: {}", v),
		Err(e) => println!("Error: ====> {}", e),
	}
}

fn main() {
	// This still presents a reasonable answer.
	let twenty = multiply_v1("10", "2");
	print(twenty); 


    // The following now provides a much more helpful error message.
    let tt = multiply_v1("t", "2");
	print(tt);

	println!("===============");
	let twe_2 = multiply_v2("10", "2");
	print(twe_2);
}