use std::fmt::Debug;

static NUM: i32 = 18;

fn coerce_static<'a> (_: &'a i32) -> &'a i32 {
	&NUM
}

fn print_it(input: impl Debug + 'static) {
	println!("'static value passed in is: {:?}", input);
}

fn main() {
	{
		let static_string = "I'm in read-only memory";
		println!("static_string: {}", static_string);
	}
	{
		let lifetime_num = 100;
		println!("coerced_static: {}", coerce_static(&lifetime_num));
	}
	println!("NUM: {} stays accessible!", NUM);


	// i is owned and contains no references, thus it's 'static:
	let i = 5;
	print_it(i);

	// oops, &i only has the lifetime defined by the scope of use_it(), so it's not 'static:
	// print_it(&i); // borrowed value does not live long enough
}