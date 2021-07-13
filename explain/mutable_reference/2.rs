fn main() {
	let mut i: i32 = 1;
	{
		let _ref_i_1: &mut i32 = &mut i;
	}
	let _ref_i_2: &mut i32 = &mut i;

	*_ref_i_2 = 2;
	
	println!("_ref_i_2: {}", _ref_i_2);
	println!("i: {}", i);
}