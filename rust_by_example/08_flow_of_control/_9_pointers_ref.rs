#[allow(unused)]
fn main() {
	let reference = &4;
	match reference {
		&val => println!("Got a value via destructuring: {:?}", val),
	}

    // To avoid the `&`, you dereference before matching.
	match *reference {
		val => println!("Got a value via destructuring: {:?}", val),
	}

	let _not_a_reference = 3;
	let ref _is_a_reference = 3;

	let value = 5;
	let mut value_mut = 6;
	// Use `ref` keyword to create a reference.
	match value {
		ref r => println!("Got a reference to a value: {:?}", r),
	}

    // Use `ref mut` similarly.
	match value_mut {
		ref mut m => {
			*m += 10;
			println!("we added 10. `value_mut`: {:?}", m);
		},
	}
}