fn main() {
	let f1 = 3.7_f32;
	let g1 = 3.0_f32;
	let h1 = -3.7_f32;
	// Returns the largest integer less than or equal to a number.
	assert_eq!(f1.floor(), 3.0);
	assert_eq!(g1.floor(), 3.0);
	assert_eq!(h1.floor(), -4.0);

	let f2 = 3.01_f32;
	let g2 = 4.0_f32;
	// Returns the smallest integer greater than or equal to a number.
	assert_eq!(f2.ceil(), 4.0);
	assert_eq!(g2.ceil(), 4.0);

	let f3 = 3.3_f32;
	let g3 = -3.3_f32;
	let h3 = 3.5_f32;
	// Returns the nearest integer to a number. Round half-way cases away from 0.0.
	assert_eq!(f3.round(), 3.0);
	assert_eq!(g3.round(), -3.0);
	assert_eq!(h3.round(), 4.0);

	let f4 = 3.7_f32;
	let g4 = 3.0_f32;
	let h4 = -3.7_f32;
	// Returns the integer part of a number.
	assert_eq!(f4.trunc(), 3.0);
	assert_eq!(g4.trunc(), 3.0);
	assert_eq!(h4.trunc(), -3.0);

	let f5 = 3.6_f32;
	let g5 = -3.6_f32;
	// Returns the fractional part of a number.
	println!("f5: {}", f5.fract());
	println!("g5: {}", g5.fract());
	assert_eq!(f5.fract(), 0.5999999);
	assert_eq!(g5.fract(), -0.5999999);
	let abs_diff_f5 = (f5.fract() - 0.6).abs();
	let abs_diff_g5 = (g5.fract() - (-0.6)).abs();
	assert!(abs_diff_f5 <= f32::EPSILON);
	assert!(abs_diff_g5 <= f32::EPSILON);
}