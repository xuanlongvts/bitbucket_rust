fn main() {
	fn long() -> () {
		()
	}
	println!("long: {:?}", long());

	fn short() {}
	println!("short: {:?}", short());

	fn return_i64() -> i64 {
		1i64
	}

	fn returns_unit() {
		1i64;
	}

	let is_i64 = {
		return_i64()
	};
	println!("is_i64: {:?}", is_i64);
	
	let is_unit = {
		returns_unit();
	};
	println!("is_unit: {:?}", is_unit);
}