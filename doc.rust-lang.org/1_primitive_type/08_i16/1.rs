fn main() {
	assert_eq!(i16::MIN, -32768);
	assert_eq!(i16::MAX, 32767);
	assert_eq!(i16::BITS, 16);

	assert_eq!(i16::from_str_radix("A", 16), Ok(10));
}