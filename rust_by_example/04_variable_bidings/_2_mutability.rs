fn main() {
	let _immutable_binding = 1;
	let mut mutable_binding = 1;

	println!("Before mutation: {}", mutable_binding);
	mutable_binding += 1;
	println!("After mutation: {}", mutable_binding);

	// cannot assign twice to immutable variable
	// _immutable_binding +=1;
}