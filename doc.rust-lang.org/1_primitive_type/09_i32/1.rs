fn main() {
	assert_eq!(i32::MIN, -2147483648);
	assert_eq!(i32::MAX, 2147483647);
	assert_eq!(i32::BITS, 32);

	assert_eq!(i32::from_str_radix("A", 16), Ok(10));
}