fn main() {
	let s_1 = "hello";
	let ptr_1 = s_1.as_ptr(); // Converts a string slice to a raw pointer.
	let ptr_1_1: *const str = s_1;
	println!("ptr_1: {:?}, ptr_1_1: {:?}", ptr_1, ptr_1_1);

	let v_1 = String::from("🗻∈🌏");
	assert_eq!(Some("🗻"), v_1.get(0..4));

	let mut v_2 = String::from("hello");
	println!("v_2: {:?}", v_2.get_mut(0..5).is_some());
	assert!(v_2.get_mut(0..5).is_some());
	assert!(v_2.get_mut(..42).is_none());
	assert_eq!(Some("he"), v_2.get_mut(..2).map(|v| &*v));
	println!(" {:?}", v_2.get_mut(..2).map(|v| &*v));
}