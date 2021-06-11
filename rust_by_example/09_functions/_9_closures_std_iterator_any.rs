fn main() {
	let vec1 = vec![1, 2, 3];
	let vec2 = vec![4, 5, 6];

	// `iter()` for vecs yields `&i32`. Destructure to `i32`.
	println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));

    // `into_iter()` for vecs yields `i32`. No destructuring required.
	println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

	println!("===========================");

	let arr1 = [1, 2, 3];
	let arr2 = [4, 5, 6];
	println!("2 in arr1: {}", arr1.iter().any(|&x| x == 2));

	// `into_iter()` for arrays unusually yields `&i32`.
	println!("2 in arr2: {}", arr2.into_iter().any(|&x| x == 2));
}