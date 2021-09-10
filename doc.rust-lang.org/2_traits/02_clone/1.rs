fn main() {
	let hi = "hi";
	assert_eq!("hi", hi.clone());

	let a: &str;
	let b: &str = "hello";
	a = b.clone();
	println!("a: {:?}", a);
}