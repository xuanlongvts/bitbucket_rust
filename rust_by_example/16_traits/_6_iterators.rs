struct Fibonacci {
	curr: u32,
	next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
	type Item = u32;

	fn next(&mut self) -> Option<u32> {
		let new_next = self.curr + self.next;

		self.curr = self.next;
		self.next = new_next;

		// Since there's no endpoint to a Fibonacci sequence, the `Iterator` 
        // will never return `None`, and `Some` is always returned.
		Some(self.curr)
	}
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
	Fibonacci {
		curr: 0,
		next: 1,
	}
}

fn main() {
	let mut sequence = 0..3;

	println!("Four consecutive `next` calls on 0..3");
	println!("> {:?}", sequence.next());
	println!("> {:?}", sequence.next());
	println!("> {:?}", sequence.next());
	println!("> {:?}", sequence.next());

	// `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    println!("Iterate through 0..3 using `for`");
	for i in 0..3 {
		println!("> i: {}", i);
	}

	// The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
	for i in fibonacci().take(4) {
		println!("> i: {}", i);
	}

 	// The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
	for i in fibonacci().skip(4).take(4) {
		println!("> i: {}", i);
	}

	let arr = [1u32, 3_u32, 3, 7];
	// The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &arr);
	for i in arr.iter() {
		println!("> i: {}", i);
	}
}