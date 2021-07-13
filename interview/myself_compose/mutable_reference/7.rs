fn main() {
	let mut i: i32 = 1;
	let ref_i: &mut i32 = &mut i;
	*ref_i = 2;
	println!("ref_i: {}", ref_i);
	println!("i: {}", i);
	i = 3; // error, assignment to borrowed `i` occurs here
	println!("ref_i: {}", ref_i);
}