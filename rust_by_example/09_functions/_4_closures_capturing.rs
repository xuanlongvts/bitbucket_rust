use std::mem;

/*
by reference: &T
by mutable reference: &mut T
by value: T
*/

fn main() {
	let color = String::from("green");

	let print = || println!("Color: {}", color);

	print();
	let _reborrow = &color;
	print();

    // A move or reborrow is allowed after the final use of `print`
	let _color_moved = color;

	let mut count = 0;
	let mut inc = || {
		count += 1;
		println!("Count: {}", count);
	};
	inc();
	inc();

	let count_reborrowed = &mut count;
	println!("count_reborrowed: {}", count_reborrowed);

    // A non-copy type.
	let movable = Box::new(3);
	let consume = || {
		println!("`movable`: {:?}", movable);
		mem::drop(movable);
	};
    // `consume` consumes the variable so this can only be called once.
	consume();

	// consume();

	println!("=====================");

	// Using move before vertical pipes forces closure to take ownership of captured variables:
	let haystack = vec![1, 2, 5];
	let contains = move |needle| haystack.contains(needle);
	println!("{}", contains(&1));
	println!("{}", contains(&5));
	println!("{}", contains(&6));
}