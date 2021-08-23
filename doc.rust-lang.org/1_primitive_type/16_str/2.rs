fn main() {
	let len = "foo".len();
	assert_eq!(3, len);
	assert_eq!("ƒoo".len(), 4); // not ƒ != f
	assert_eq!("ƒoo".chars().count(), 3);

	assert!("".is_empty());
	assert!(!"hi".is_empty());

	let s_1 = "Löwe 老虎 Léopard";
	assert!(s_1.is_char_boundary(0));
	assert!(s_1.is_char_boundary(6));
	assert!(s_1.is_char_boundary(s_1.len()));
	assert!(!s_1.is_char_boundary(2)); // second byte of `ö`
	assert!(!s_1.is_char_boundary(8)); // third byte of `老`

	let bytes_1 = "bors".as_bytes();
	assert_eq!(b"bors", bytes_1);

	let mut s_2 = String::from("hello");
	let bytes_2 = unsafe{ s_2.as_bytes_mut() };
	assert_eq!(bytes_2, b"hello");

	let mut s_3 = String::from("🗻∈🌏");
	unsafe { 
		let bytes = s_3.as_bytes_mut();

		bytes[0] = 0xF0;
		bytes[1] = 0x9F;
		bytes[2] = 0x8D;
		bytes[3] = 0x94;
	};
	assert_eq!("🍔∈🌏", s_3);
}
