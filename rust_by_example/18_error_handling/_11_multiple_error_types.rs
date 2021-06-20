fn double_first(vec: Vec<&str>) -> i32 {
	let first = vec.first().unwrap();

	2 * first.parse::<i32>().unwrap()
}

fn main() {
	let numbers = vec!["10", "20", "30"];
	let empty = vec![];
	let strings = vec!["aa", "bb", "cc"];

	println!("1. The first doubled is: {}", double_first(numbers));

	println!("2. The first doubled is: {}", double_first(empty));

	println!("3. The first doubled is: {}", double_first(strings));
}