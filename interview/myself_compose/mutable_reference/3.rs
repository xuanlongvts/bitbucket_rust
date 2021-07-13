fn main() {
	let mut i: i32 = 1;
	let ref_i: &mut i32 = &mut i;
	*ref_i = 2;
	println!("i: {}", i); // error, immutable borrow occurs here
	println!("ref_i: {}", ref_i);
}