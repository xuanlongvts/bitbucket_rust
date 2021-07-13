struct Foo {
	i: i32
}

fn main() {
	let j = {
		let x = Foo {
			i: 10
		};
		let y = &x;
		y.i
	};

	println!("j: {}", j);
}