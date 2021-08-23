fn main() {
	let my_num: i32 = 10;

	let my_num_prt_1 = &my_num;
	let my_num_prt_2: *const i32 = &my_num;
	println!("my_num: {}", my_num);
	println!("my_num_prt_1: {:?}", my_num_prt_1); // 10
	println!("my_num_prt_2: {:?}", my_num_prt_2); // 0x7ffee3f93164

	let mut my_speed: i32 = 88;
	let my_speed_ptr: *mut i32 = &mut my_speed;
	println!("my_speed_ptr: {:?}", my_speed_ptr); // 0x7ffeef7701fc

	println!("=======================");
	// To get a pointer to a boxed value, dereference the box:
	let my_num_1: Box<i32> = Box::new(10);
	let my_num_prt_3: *const i32 = &*my_num_1;
	println!("my_num_1: {}", my_num_1); // 10
	println!("my_num_prt_3: {:?}", my_num_prt_3); // 0x7faaf6405f50

	let mut my_speed_1: Box<i32> = Box::new(88);
	let my_speed_ptr_4: *mut i32 = &mut *my_speed_1;
	println!("my_speed_ptr_4: {:?}", my_speed_ptr_4) // 0x7f98fc405f60
}