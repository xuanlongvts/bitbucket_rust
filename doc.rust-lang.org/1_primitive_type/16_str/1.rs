use std::slice;
use std::str;

fn main() {
	let hello: &'static str = "hello, world!";
	println!("hello: {:?}", hello);

	let story = "Once upon a time...";
	let ptr = story.as_ptr();
	let len = story.len();

	let s = unsafe {
	    // First, we build a &[u8]...
		let slice = slice::from_raw_parts(ptr, len);

	    // ... and then convert that slice into a string slice
		str::from_utf8(slice)
	};
	assert_eq!(s, Ok(story));
}