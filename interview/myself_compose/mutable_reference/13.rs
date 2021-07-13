fn main() {
	let mut i: i32 = 1;
	let j =  {
		let x = &i;
		let y = &x;

		&**y
	};

	println!("j: {}", j);
}