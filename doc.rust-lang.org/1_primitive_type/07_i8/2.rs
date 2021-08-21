fn main() {
	// 0x is prefix in hexadecimal
	// hexadecimal, 0-9, a-f. a ~ 10
	let n1 = -0x7ei8; // hexadecimal. e ~ 14. 714 ==> -((7 * (16 ^ 1)) + (14 * (16 ^ 0))) = -126
	println!("n1: {:?}", n1);
	let m1: i8 = 0xa; // hexadecimal a ~ 10. decimal of a is 1010, 8 bit is 00001010

	println!("m1: {:?}", m1.rotate_right(1)); // 5 because 00001010 --> 00000101
	println!("m1: {:?}", m1.rotate_right(2)); // -126 because 00001010 --> 10000010 (Chú ý: Trong 32 bit, bit đầu tiên được sử dụng để xác định dấu (sign) của số, nếu bit này là 1 tương ứng với dấu trừ ( - ), nếu bit này là 0 tương ứng với dấu cộng ( + ))
	println!("m1: {:?}", m1.rotate_right(3)); // 65 because 00001010 --> 010000010
	assert_eq!(n1.rotate_left(2), m1);

	let m2: i8 = -0xa;
	// hexadecimal a ~ 10 (00001010), có dấu -, nên là -10, được biểu diễn bởi 11110110
	// theo phương pháp bù 2 https://vi.wikipedia.org/wiki/B%C3%B9_2 
	// và cộng bit http://vnhow.vn/howto/cach-cong-hai-so-nhi-phan
	println!("m2: {:?}", m2.rotate_left(1)); // -19 (11101101) [-(2 ** 7) + (2 ** 6) + (2 ** 5) + (2 ** 3) + (2 ** 2) + 1]
	println!("m2: {:?}", m2.rotate_left(2)); // -37 (11011011) [-(2 ** 7) + (2 ** 6) + (2 ** 4) + (2 ** 3) + (2 ** 1) + 1]

	let n3 = 0xai8; // 10
	let m3: i8 = -0x7e; // -126
	assert_eq!(n3.rotate_right(2), m3);
}