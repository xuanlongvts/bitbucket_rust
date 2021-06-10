use std::convert::From;

#[derive(Debug)]
struct Number {
	value: i32,
}

impl From<i32> for Number {
	fn from(item: i32) -> Self {
		Number { value: item }
	}
}

fn main() {
	let my_str = "hello";
	let my_string = String::from(my_str);
	println!("my_string: {}", my_string);

	let num = Number::from(30);
	println!("My number is: {:?}", num);

	let int = 5;
	// Try remove the type declaration
	let num2: Number = int.into();
	println!("Num2 is: {:?}", num2);
}