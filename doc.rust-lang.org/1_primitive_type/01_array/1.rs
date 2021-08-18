fn main() {
	let mut array: [i32; 3] = [0; 3];

	array[1] = 1;
	array[2] = 2;

	// println!("{:?}", array);
	assert_eq!([1, 2], &array[1..]);

	for i in array {
		print!("{}, ", i);
	}
	println!("\n==================");
	for i in &array {
		print!("{}, ", i);
	}
}