fn main() {
	assert_eq!(i64::MIN, -9223372036854775808);
	assert_eq!(i64::MAX, 9223372036854775807);
	assert_eq!(i64::BITS, 64);
	assert_eq!(i64::from_str_radix("A", 16), Ok(10));

	let n1 = 0b100_0000i64;
	assert_eq!(n1.count_ones(), 1);
}