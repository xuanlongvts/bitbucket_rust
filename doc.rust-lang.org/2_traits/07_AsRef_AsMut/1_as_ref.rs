fn main() {
	fn is_hello<T: AsRef<str>>(s: T) {
		assert_eq!("hello", s.as_ref());
	}

	let s1 = "hello";
	is_hello(s1);

	let s2 = "hello".to_string();
	is_hello(s2);
}