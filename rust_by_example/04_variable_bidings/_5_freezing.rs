fn main() {
	let mut _mutable_integer = 7i32;

	{
		let _mutable_integer = _mutable_integer;

		// error
		// _mutable_integer = 50;

		// `_mutable_integer` goes out of scope
	}
	_mutable_integer = 3;
	println!("_mutable_integer: {}", _mutable_integer);
}