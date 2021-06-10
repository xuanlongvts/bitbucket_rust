#![allow(unreachable_code)]

fn main() {
	let mut count = 0u32;
    println!("Let's count until infinity!");

	// Infinite loop
	loop {
		count += 1;
		if count == 3 {
			println!("three");
			continue;
		}
		println!("{}", count);
		if count == 5 {
			println!("Ok, that's enough");

            // Exit this loop
			break;
		}
	}

	println!("=========================== Nesting and labels");

	'outer: loop {
 		println!("Entered the outer loop");

		'inner: loop {
			println!("Entered the inner loop");

			break 'outer;
			// ------------ any code following this expression is unreachable
		}
		println!("This point will never be reached");
	}
	println!("Exited the outer loop");

	println!("=========================== Returning from loops");
	let mut i = 0;
	let result = loop {
		i += 1;

		if i == 10 {
			break i * 2;
		}
	};
	println!("i = {}", result);
}