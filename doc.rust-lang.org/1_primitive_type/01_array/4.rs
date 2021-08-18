use std::array::IntoIter;

fn main() {
	let array: [i32; 3] = [0; 3];

	// This iterates by reference:
	for item in array.iter() {
		let x: &i32 = item;
		println!("{}", x);
	}

	println!("===================");
	// This iterates by value:
	for item in IntoIter::new(array) {
		let x: i32 = item;
		println!("{}", x);
	}

	println!("===================");
	// This iterates by value:
	for item in array {
		let x: i32 = item;
		println!("{}", x);
	}

	println!("===================");
	// IntoIter can also start a chain. This iterates by value:
	for item in IntoIter::new(array).enumerate() {
		let (i, v): (usize, i32) = item;
		println!("array[{}] = {}", i, v);
	}

}