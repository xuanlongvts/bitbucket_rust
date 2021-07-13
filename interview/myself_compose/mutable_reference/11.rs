fn main() {
	let mut i: i32 = 1;
	let ref_i = &mut i;
	let ref_i_2 = ref_i;

	*ref_i_2 = 2;

	println!("ref_i_2: {}", ref_i_2);
	println!("i: {}", i);
	// println!("ref_i: {}", ref_i); // error, value borrowed here after move
}