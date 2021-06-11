fn apply<F>(f: F) where F: Fn() {
	f();
}

fn main() {
	let x = 7;

	 // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
	let print = || println!("print closure: {}", x);

	apply(print);

	println!("x1: {}", x);
	println!("x2: {}", x);
}