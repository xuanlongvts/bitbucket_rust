use std::num::ParseIntError;

// result out option
fn double_first_v1(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
	vec.first().map(|first| {
		first.parse::<i32>().map(|n| n * 2)
	})
}

// option out result
// There are times when we'll want to stop processing on errors (like with ?) but keep going when the Option is None. 
// A couple of combinators come in handy to swap the Result and Option.

// map_or ===> Returns the provided default result (if none), or applies a function to the contained value (if any).
/*
let x = Some("foo");
assert_eq!(x.map_or(42, |v| v.len()), 3);

let x: Option<&str> = None;
assert_eq!(x.map_or(42, |v| v.len()), 42);
*/

fn double_first_v2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
	let opt = vec.first().map(|first| {
		first.parse::<i32>().map(|n| n * 2)
	});

	opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
	let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

	println!("1. Double: {:?}", double_first_v1(numbers));
	println!("2. Double: {:?}", double_first_v1(empty));
	println!("3. Double: {:?}", double_first_v1(strings));

	println!("==========");
	let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
	println!("1. Double: {:?}", double_first_v2(numbers));
	println!("2. Double: {:?}", double_first_v2(empty));
	println!("3. Double: {:?}", double_first_v2(strings));
}