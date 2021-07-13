fn main() {
	let clo_add_one_1 = |x| {1 + x};
	println!("The sum of 5 plus 1 is: {}", clo_add_one_1(5));

	let clo_add_one_2 = |x: i32| -> i32 {1 + x};
	fn fn_clo_add_one_2(x: i32) -> i32 {
		1 + x
	}
	println!("closure: {}, function: {}", clo_add_one_2(5), fn_clo_add_one_2(5));

	let x: i32 = 10;
	let printer_1 = || { println!("x is: {}", x); };
	printer_1();

	let mut y: i32 = 11;
	let printer_2 = || { println!("y is: {}", y); };
	// y = 6; error
}

/*
Moving closures

Rust has a second type of closure, called a moving closure. Moving closures are indicated using the move keyword (e.g., move || x * x). 
The difference between a moving closure and an ordinary closure is that a moving closure always takes ownership of all variables that it uses. 
Ordinary closures, in contrast, just create a reference into the enclosing stack frame. 
Moving closures are most useful with Rust's concurrency features, and so we'll just leave it at this for now. 
We'll talk about them more in the "Threads" section of the guide.
*/