fn main() {
	let array: [i32; 3] = [0; 3];
	
	for item in array.into_iter().enumerate() {
		let (i, x): (usize, &i32) = item;
		println!("array[{}] = {} ", i, x);
	}
	println!("=======================");
	for item in IntoIterator::into_iter(array).enumerate() {
		let (i, x): (usize, i32) = item;
		println!("array[{}] = {} ", i, x);
	}
	println!("=======================");
	for (i, v) in array.iter().enumerate() {
		println!("array[{}] = {} ", i, v);
	}
	println!("=======================");
	for item in array.iter().enumerate() {
		let (i, v): (usize, &i32) = item;
		println!("array[{}] = {} ", i, v);
	}
}