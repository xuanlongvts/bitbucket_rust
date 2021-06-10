#![allow(unused)]
fn main() {
    // variable binding
	let x = 5;

	// expression
	x;
	x + 1;
	15;
	println!("x = {}", x);

	let y = 5u32;
	let z = {
		let y_squared = y * y;
		let y_cube = y_squared * y;

	    // This expression will be assigned to `z`
		y_cube + y_squared + y
	};
	let w = {
        // The semicolon suppresses this expression and `()` is assigned to `w`
		2 * y;
	};

	println!("y = {:?}", y);
	println!("z = {:?}", z);
	println!("w = {:?}", w);
}