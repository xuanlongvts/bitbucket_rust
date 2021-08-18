#![allow(unused)]

struct Cat {
	name: String
}

fn test<T>(t: T) {
}

fn main() {
	let c = Cat { name: String::from("hi c") };
	test(c);
	// test(c); // value used here after move

	let d = Cat {
		name: String::from("hi d")
	};
	let e = d;
	// let f = d; // value used here after move

	let g = Cat {
		name: String::from("hi G")
	};
	test(&g);
	test(&g);

	let h = Cat {
		name: String::from("hi H")
	};
	let i = &h;
	let g = i;
	let k = &h;
}