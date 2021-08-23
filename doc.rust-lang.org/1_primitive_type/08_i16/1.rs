fn main() {
	assert_eq!(i16::MIN, -32768);
	assert_eq!(i16::MAX, 32767);
	assert_eq!(i16::BITS, 16);

	assert_eq!(i16::from_str_radix("A", 16), Ok(10));

	let n1 = 0b100_0000i16;
	// Returns the number of ones in the binary representation of self.
	assert_eq!(n1.count_ones(), 1);

	// Returns the number of zeros in the binary representation of self.
	assert_eq!(i16::MAX.count_zeros(), 1);
}