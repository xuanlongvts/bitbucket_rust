fn main() {
	let mut i: i32 = 1;
	let ref_i = &mut i;
	let ref_i_2 = ref_i;

	*ref_i = 2; // error, value used here after move
}