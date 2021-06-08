use std::fmt;

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let vec = &self.0;

		write!(f, "[")?;
		for(count, v) in vec.iter().enumerate() {
			// println!("count = {}, v = {}", count, v);

			if count != 0 {
				write!(f, ", ")?;
			}
			write!(f, "{}", v)?;
		}
		write!(f, "]")
	}
}

struct ListMore(Vec<i32>);

impl fmt::Display for ListMore {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let vec = &self.0;

		write!(f, "[")?;
		for(count, v) in vec.iter().enumerate() {
			if count != 0 {
				write!(f, ", ")?;
			}
			write!(f, "{}: {}", v,  v + 1)?;
		}
		write!(f, "]")
	}
}

fn main() {
	let v1 = List(vec![1, 2, 3]);
	println!("v1 = {}: ", v1);

	let v2 = ListMore(vec![2, 4, 6]);
	println!("v2 = {}: ", v2);
}