fn main() {
	let mut i: i32 = 1;
	let ref_i: &mut i32 = &mut i;
	*ref_i = 2;
	println!("ref_i: {}, i: {}", ref_i, i); // error, i immutable borrow occurs here
}