fn main() {
	assert_eq!(isize::MIN, -9223372036854775808);
	assert_eq!(isize::MAX, 9223372036854775807);
	assert_eq!(isize::BITS, 64);
	assert_eq!(isize::from_str_radix("A", 16), Ok(10));
}