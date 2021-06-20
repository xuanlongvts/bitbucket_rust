use std::num::ParseIntError;

type AliasResut<T> = Result<T, ParseIntError>;

fn multiply(f_n: &str, s_n: &str) -> AliasResut<i32> {
	let f_number = f_n.parse::<i32>()?;
	let s_number = s_n.parse::<i32>()?;

	// can we use try! note: `#[warn(deprecated)]` on by default
	// let f_number = try!(f_n.parse::<i32>());
	// let s_number = try!(s_n.parse::<i32>());

	Ok(f_number * s_number)
}

fn print(result: AliasResut<i32>) {
	match result {
		Ok(v) => println!("value: {}", v),
		Err(e) => println!("Error: {}", e),
	}
}

fn main() {
	let twe_1 = multiply("10", "2");
	print(twe_1);

	let twe_2 = multiply("tt", "2");
	print(twe_2);
}