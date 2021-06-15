struct Empty;
struct Null;

trait DoubleDrop<T> {
	// Define a method on the caller type which takes an additional single parameter `T` and does nothing with it.
	fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U{
	fn double_drop(self, _: T) {}
}

fn main() {
	let empty = Empty;
	let null = Null;

	empty.double_drop(null);

	// empty;
	// null;
}