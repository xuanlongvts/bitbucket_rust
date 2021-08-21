fn main() {
	let v = vec!['h', 'e', 'l', 'l', 'o'];
	assert_eq!(20, v.len() * std::mem::size_of::<char>());

	let s = String::from("hello");
	assert_eq!(5, s.len() * std::mem::size_of::<u8>());

	let mut chars = "é".chars();
	// U+00e9: 'latin small letter e with acute'
	// assert_eq!(Some('\u{00e9}'), chars.next());
	// assert_eq!(None, chars.next());

	// U+0065: 'latin small letter e'
	assert_eq!(Some('\u{0065}'), chars.next());
	
	// U+0301: 'combining acute accent'
	assert_eq!(Some('\u{0301}'), chars.next());
	assert_eq!(None, chars.next());

	// Another implication of the 4-byte fixed size of a char is that per-char processing can end up using a lot more memory:
	let s = String::from("love: ❤️");
	let v: Vec<char> = s.chars().collect();
	assert_eq!(12, std::mem::size_of_val(&s[..]));
	assert_eq!(32, std::mem::size_of_val(&v[..]));
}