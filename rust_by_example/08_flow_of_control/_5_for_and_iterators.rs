fn main() {
	let mains1 = vec!["Bob", "Frank", "Ferris"];

	// iter - This borrows each element of the collection through each iteration. 
	// Thus leaving the collection untouched and available for reuse after the loop.
	for name in mains1.iter() {
		match name {
			&"Ferris" => println!("There is a rustacean among us!"),
			_ => println!("Hello {}", name),
		}
	}
	println!("mains1: {:?}", mains1);

	// into_iter - This consumes the collection so that on each iteration the exact data is provided. 
	// Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
	for name in mains1.into_iter() {
		match name {
			"Ferris" => println!("There is a rustacean among us!"),
			_ => println!("Hello {}", name),
		}
	}
	// error value borrowed here after move
	// println!("mains1: {:?}", mains1);

	// iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.
	let mut mains2 = vec!["Bob", "Frank", "Ferris"];
	for name in mains2.iter_mut() {
		*name = match name {
			&mut "Ferris" => "There is a rustacean among us!",
			_ => "Hello",
		}
	}
	println!("mains2: {:?}", mains2);

}