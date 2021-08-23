fn main() {
	assert_eq!(i128::MIN, -170141183460469231731687303715884105728);
	assert_eq!(i128::MAX, 170141183460469231731687303715884105727);
	assert_eq!(i128::BITS, 128);
	assert_eq!(i128::from_str_radix("A", 16), Ok(10));
}