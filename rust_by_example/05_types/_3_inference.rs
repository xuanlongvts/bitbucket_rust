fn main() {
	let elem = 5u8;

	let mut vec = Vec::new();
	vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)

	println!("{:?}", vec);
}