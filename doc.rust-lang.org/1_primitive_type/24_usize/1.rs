fn main() {
	assert_eq!(usize::MIN, 0);
	assert_eq!(usize::MAX, 18446744073709551615);
	assert_eq!(usize::BITS, 64);

	assert_eq!(usize::from_str_radix("A", 16), Ok(10));
}