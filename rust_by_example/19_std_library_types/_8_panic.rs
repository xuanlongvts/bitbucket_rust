fn division(dividend: i32, divisor: i32) -> i32 {
	if divisor == 0 {
		panic!("Error: =====> division by zero");
	} else {
		dividend / divisor 
	}
}

fn main() {
	// Heap allocated integer
	let _x = Box::new(0i32);

	division(3, 0);

	println!("This point won't be reached!");

	// `_x` should get destroyed at this point
}
