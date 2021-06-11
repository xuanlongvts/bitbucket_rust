use std::mem;

/*
Fn: the closure captures by reference (&T)
FnMut: the closure captures by mutable reference (&mut T)
FnOnce: the closure captures by value (T)
*/

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where F: FnOnce() {
	f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
	f(3)
}

fn main() {
	let greeting = "Hello";

	// A non-copy type.
    // `to_owned` creates owned data from borrowed one
	let mut farewell = "goodbye".to_owned();

	// Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
	let diary = || {
		println!("I said: {}", greeting);
		farewell.push_str("!!!");
		
		println!("Then I screamed: {}.", farewell);
		println!("Now I can sleep, ZZZZ");

		// Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
		mem::drop(farewell);
	};

	apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
	let double = |x| 2 * x;
	println!("3 doubled: {}", apply_to_3(double));
}