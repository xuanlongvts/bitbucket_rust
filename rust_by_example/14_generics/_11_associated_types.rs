struct Container(i32, i32);

trait Contains {
	type A;
	type B;

	fn contains(&self, _: &Self::A, _: &Self::B) -> bool;

	fn first(&self) -> i32;

	fn last(&self) -> i32;
}

impl Contains for Container {
	type A = i32;
	type B = i32;

	// `&Self::A` and `&Self::B` are also valid here.
	fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
		(&self.0 == number_1) && (&self.1 == number_2)
	}

	// fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
	// 	(&self.0 == number_1) && (&self.1 == number_2)
	// }

	fn first(&self) -> i32 {
		self.0
	}

	fn last(&self) -> i32 {
		self.1
	}
}

fn difference<C: Contains>(contains: &C) -> i32 {
	contains.last() - contains.first()
}

fn main() {
	let number_1 = 3;
	let number_2 = 10;

	let contains = Container(number_1, number_2);

	println!("Does container contain {} and {}: {}", &number_1, &number_2, contains.contains(&number_1, &number_2));

	println!("First number: {}", contains.first());
	println!("Last number: {}", contains.last());

	println!("Different number: {}", difference(&contains));
}
