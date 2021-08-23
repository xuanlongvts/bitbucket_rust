use std::rc::Rc;

fn main() {
	let vec_1 = vec![1, 2, 3];
	let init_slice = &vec_1[..];
	println!("init_slice: {:?}", init_slice);

	let str_slice: &[&str] = &["one", "two", "three"];
	println!("str_slice: {:?}", str_slice);

	let mut x_1 = [1, 2, 3];
	let x_2 = &mut x_1[..];
	x_2[1] = 7;
	assert_eq!(x_2, &[1, 7, 3]);
	println!("x_2: {:?}", x_2);
	println!("x_1: {:?}", x_1);

	// As slices store the length of the sequence they refer to, they have twice the size of pointers to Sized types. 
	// Also see the reference on dynamically sized types.
	let pointer_size = std::mem::size_of::<&u8>();
	println!("pointer_size: {:?}", pointer_size);
	assert_eq!(2 * pointer_size, std::mem::size_of::<&[u8]>());
	assert_eq!(2 * pointer_size, std::mem::size_of::<*const [u8]>());
	assert_eq!(2 * pointer_size, std::mem::size_of::<Box<[u8]>>());
	assert_eq!(2 * pointer_size, std::mem::size_of::<Rc<[u8]>>());
}