#[allow(unused)]
fn main() {
	/* ------------ way 1 */
	// #[derive(Debug)]
	#[derive(Debug, Copy, Clone)]
	struct Foo;

	let x = Foo;
	let y = x; // x has moved into y and x cannot use, if we use #[derive(Debug, Copy, Clone)] then y copy from x
	
	println!("x: {:?}", x);

	/* ------------ way 2 */
	#[derive(Debug)]
	struct MyStruct;

	impl Copy for MyStruct {};
	impl Clone for MyStruct {
		fn clone(&self) -> MyStruct {
			*self
		}
	};
	let a = MyStruct;
	let b = a;
	println!("a = {:?}", a);
}