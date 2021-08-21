fn main() {
	let f1 = 3.7_f64;
	let g1 = 3.0_f64;
	let h1 = -3.7_f64;
	// Returns the largest integer less than or equal to a number.
	assert_eq!(f1.floor(), 3.0);
	assert_eq!(g1.floor(), 3.0);
	assert_eq!(h1.floor(), -4.0);

	let f2 = 3.01_f64;
	let g2 = 4.5_f64;
	let h2 = -3.5_f64;
	// Returns the smallest integer greater than or equal to a number.
	assert_eq!(f2.ceil(), 4.0);
	assert_eq!(g2.ceil(), 5.0);
	assert_eq!(h2.ceil(), -3.0);

	let f3 = 3.3_f64;
	let g3 = -3.3_f64;
	let h3 = -3.5_f64;
	assert_eq!(f3.round(), 3.0);
	assert_eq!(g3.round(), -3.0);
	assert_eq!(h3.round(), -4.0);

	let f4 = 3.7_f64;
	let g4 = 3.0_f64;
	let h4 = -3.7_f64;
	// Returns the integer part of a number.
	assert_eq!(f4.trunc(), 3.0);
	assert_eq!(g4.trunc(), 3.0);
	assert_eq!(h4.trunc(), -3.0);
}