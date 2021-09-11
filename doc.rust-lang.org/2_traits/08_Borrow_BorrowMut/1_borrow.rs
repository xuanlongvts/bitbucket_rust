use std::borrow::Borrow;

fn main() {
	fn check<T: Borrow<str>>(s: T) {
		assert_eq!("hello", s.borrow());
	}
	let s1 = "hello".to_string();
	check(s1);

	let s2 = "hello";
	check(s2);
}