use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
	let mut hasher = DefaultHasher::new();
	7920.hash(&mut hasher);
	println!("Hash is: {}", hasher.finish());

	let mut hasher2 = DefaultHasher::new();
	let numbers = [6, 28, 496, 8128];
	Hash::hash_slice(&numbers, &mut hasher2);
	println!("Hash is: {:x}!", hasher2.finish());
}