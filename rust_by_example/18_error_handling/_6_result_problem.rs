use std::num::ParseIntError;

fn multiply_v1(first_num: &str, second_num: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
	let first = first_num.parse::<i32>().unwrap();
	let second = second_num.parse::<i32>().unwrap();
	first * second
}

fn multiply_v2(num: &str) -> Result<(), ParseIntError> {
	let number = match num.parse::<i32>() {
		Ok(n)	=> n,
		Err(e)	=> return Err(e),
	};
	println!("Parse v2: {}", number);
	Ok(())
}

fn main() {
	let twenty = multiply_v1("10", "2");
	println!("Double is: {}", twenty);

	let resutl_v2 = multiply_v2("5");
	println!("resutl_v2 ok: {:?}", resutl_v2);

	let _resutl_v2 = multiply_v2("Error");
	println!("resutl_v2 error: {:?}", _resutl_v2);

	// let tt = multiply_v1("t", "2");
	// println!("Can't parse: {}", tt);
}