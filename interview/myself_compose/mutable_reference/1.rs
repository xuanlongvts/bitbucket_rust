fn main() {
	let mut i: i32 = 1;
	let ref_i: &mut i32 = &mut i;
	println!("i: {}", i); // error, immutable borrow occurs here
	*ref_i = 2;
}