fn main() {
	let my_speed_1: Box<i32> = Box::new(88);
	let my_speed_2: *mut i32 = Box::into_raw(my_speed_1);
	// println!("my_speed_1: {:?}", my_speed_1); // value borrowed here after move
	println!("my_speed_2: {:?}", my_speed_2); // 0x7f9cbdc05f50

	unsafe {
		let a = drop(Box::from_raw(my_speed_2));
		println!("a: {:?}", a);
	}

	/*
	Instead of coercing a reference to a raw pointer, you can use the macros ptr::addr_of! (for *const T) and ptr::addr_of_mut! (for *mut T). 
	These macros allow you to create raw pointers to fields to which you cannot create a reference (without causing undefined behaviour), such as an unaligned field. 
	This might be necessary if packed structs or uninitialized memory is involved.
	*/
	#[derive(Debug, Default, Copy, Clone)]
	#[repr(C, packed)]
	struct S {
		aligned: u8,
		unaligned: u32,
	}
	let s1 = S::default();
	println!("s1: {:?}", s1);
	let p1 = std::ptr::addr_of!(s1.unaligned);
	println!("p1: {:?}", p1); // 0x7ffeeabe31a9

	println!("=============");
	// is_null(self) -> bool Returns true if the pointer is null.
	let a1: &str = "Follow the rabbit";
	let ptr: *const u8 = a1.as_ptr();
	println!("ptr: {:?}", ptr);
	assert_eq!(ptr.is_null(), false);

}