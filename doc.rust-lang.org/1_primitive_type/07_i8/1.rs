fn main() {
	// The smallest value that can be represented by this integer type.
	assert_eq!(i8::MIN, -128);

	// The largest value that can be represented by this integer type.
	assert_eq!(i8::MAX, 127);

	// The size of this integer type in bits.
	assert_eq!(i8::BITS, 8);

	// Converts a string slice in a given base to an integer.
	// This function panics if radix is not in the range from 2 to 36.
	// hexadecimal 0-9 and a - f
	assert_eq!(i8::from_str_radix("A", 16), Ok(10));
	assert_eq!(i8::from_str_radix("F", 16), Ok(15));
	// assert_eq!(i8::from_str_radix("G", 16), Ok(16));

	// Returns the number of ones in the binary representation of self.
	let n = 0b100_000i8;
	assert_eq!(n.count_ones(), 1);

	// Returns the number of zeros in the binary representation of self.
	println!("{:?}", i8::MAX);
	assert_eq!(i8::MAX.count_zeros(), 1);

	// Returns the number of leading zeros in the binary representation of self.
	let n = -1i8;
	assert_eq!(n.leading_zeros(), 0);

	// Returns the number of trailing zeros in the binary representation of self.
	let n = -4i8;
	assert_eq!(n.trailing_zeros(), 2);

	// Returns the number of leading ones in the binary representation of self.
	let n = -1i8;
	assert_eq!(n.leading_ones(), 8);
}