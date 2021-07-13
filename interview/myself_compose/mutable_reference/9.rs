fn main() {
	let mut i: i32 = 1;
	let _ref_i_1: &mut i32 = &mut i; // first mutable borrow occurs here
	let _ref_i_2: &mut i32 = &mut i; // error, second mutable borrow occurs here

	*_ref_i_1 = 2; // first borrow later used here
}