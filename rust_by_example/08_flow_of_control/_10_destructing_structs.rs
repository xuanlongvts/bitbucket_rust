fn main() {
	struct Foo {
		x: (u32, u32),
		y: u32,
	}
	let foo = Foo { x: (1, 2), y: 3};
	match foo {
		Foo { x: (1, b), y} => println!("First of x is 1, b = {},  y = {} ", b, y),
		// you can destructure structs and rename the variables,
        // the order is not important
		Foo { y: 2, x: i} => println!("y is 2, x = {:?}", i),
		Foo { y, .. } => println!("y = {}, we don't care about x", y),
	}
}