fn main() {
	let x: i32 = 1;
	// x = 5; // error

	let mut y: i32 = 2; // #[warn(unused_assignments)]
	y = 3;
	println!("x: {}, y: {}", x, y);

	let mut i: i32 = 1;
	let ref_i: &mut i32 = &mut i;
	// println!("i: {}", i);
	*ref_i = 2;
	println!("ref_i 1: {}", ref_i);
	println!("i: {}", i);
	i = 3; // error, assignment to borrowed `i` occurs here
	println!("i: {}", i); // ok
	// println!("ref_i: {}", ref_i); // borrow later used here
}