fn main() {
	let mut i: i32 = 1;
	
	let j = {
		let x = &mut i; // reference but not copy
		let y = &x; // error, borrowed value does not live long enough
		&**y
	};

	println!("j: {}", j);
}