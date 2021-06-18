use std::mem::drop;

// A unit struct without resources
#[derive(Debug, Clone, Copy)]
struct Unit;

// A tuple struct with resources that implements the `Clone` trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
	// Instantiate `Unit`
	let unit = Unit;

	// Copy `Unit`, there are no resources to move
	let copied_unit = unit;

	// Both `Unit`s can be used independently
	println!("origional: {:?}", unit);
	println!("copy: {:?}", copied_unit);

    // Instantiate `Pair`
	let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Move `pair` into `moved_pair`, moves resources
	let moved_pair = pair;
	println!("moved: {:?}", moved_pair);

	// Error! `pair` has lost its resources
	// println!("original: {:?}", pair);

	// Clone `moved_pair` into `cloned_pair` (resources are included)
	let cloned_pair = moved_pair.clone();
	println!("cloned_pair: {:?}", cloned_pair);
	
	println!("moved_pair before: {:?}", moved_pair);
	
	drop(moved_pair);

	// Error! `moved_pair` has been dropped
	// println!("moved_pair after: {:?}", moved_pair);

	println!("cloned_pair 222: {:?}", cloned_pair);
}