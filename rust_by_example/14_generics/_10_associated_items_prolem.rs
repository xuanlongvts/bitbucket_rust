struct Container(i32, i32);

trait Contains<A, B> {
	fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`.

	// the same
	// fn contains(&self, &A,  &B) -> bool; // Explicitly requires `A` and `B`.

	fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.

	fn last(&self) -> i32; // Doesn't explicitly require `A` or `B`.
}

impl Contains<i32, i32> for Container {
	fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
		(&self.0 == number_1) && (&self.1 == number_2)
	}

	// Grab the first number.
	fn first(&self) -> i32 {
		self.0
	}

	// Grab the last number.
	fn last(&self) -> i32 {
		self.1
	}
}

fn difference<A, B, C>(container: &C) -> i32 where C: Contains<A, B> {
	container.last() - container.first()
}

fn main() {
	let number_1 = 3;
	let number_2 = 10;

	let container = Container(number_1, number_2);
	println!("Does the container contains {} and {} : {}", number_1, number_2, container.contains(&number_1, &number_2));

	println!("First number: {}", container.first());
	println!("Last number: {}", container.last());

	println!("The different is: {}", difference(&container));
}