struct Foo {
	i: i32
}

fn main() {
	let j = {
		let x = Foo {
			i: 10
		};
		let y = &x; // error, borrowed value does not live long enough
		&y.i
	};

	println!("j: {}", j);
}