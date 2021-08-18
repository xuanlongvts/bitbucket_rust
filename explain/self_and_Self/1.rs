/*
In Rust, self and Self mean two difference things.

self refers to the current module or object.
Self refers to the type of the current module or object.

One of the examples that clarifies this difference is the implementation of PartialEq. 
Let’s imagine that we want to implement PartialEq for an struct.

------------
Self, with a capital "S", is used to refer to the implementing type within traits and implementations.

Self can only be used as the first segment, without a preceding ::.
*/

struct Employee {
	name: String
}

impl PartialEq for Employee {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}
// ----------------------------------
trait T {
	type Item;
	const C: i32;

	// `Self` will be whatever type that implements `T`.
	fn new() -> Self;

	// `Self::Item` will be the type alias in the implementation.
	fn f(&self) -> Self::Item;
}
struct S;
impl T for S {
	type Item = i32;
	const C: i32 = 9;

	fn new() -> Self { // `Self` is the type `S`.
		S
	}
 
	fn f(&self) -> Self::Item { // `Self::Item` is the type `i32`.
		Self::C 				// `Self::C` is the constant value `9`.
	}
}

fn main() {
	let emp1 = Employee {
		name: String::from("long")
	};
	let emp2 = Employee {
		name: String::from("long le")
	};
	let emp3 = Employee {
		name: String::from("long")
	};

	println!("1 == 1: {:?}", emp1.eq(&emp1));
	println!("1 == 2: {:?}", emp1.eq(&emp2));
	println!("1 == 3: {:?}", emp1.eq(&emp3));

	println!("=====================");
	let get_s = S;
	println!("get num: {:?}", get_s.f());
}