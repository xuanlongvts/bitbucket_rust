fn main() {
	let a_binding;
	{
		let x = 2;
		a_binding = x * x;
	}
	println!("a binding: {}", a_binding);

	let another_binding;
	// Error! Use of uninitialized binding, consider giving `another_binding` a type
	// println!("another binding: {}", another_binding);

	another_binding = 1;
    println!("another binding: {}", another_binding);
}