fn main() {
	let tup_1 = ("hello", 5, 'c');
	assert_eq!(tup_1.0, "hello");
	assert_eq!(tup_1.1, 5);
	assert_eq!(tup_1.2, 'c');

	fn calculate_point() -> (i32, i32) {
		(4, 5)
	}
	let point = calculate_point();
	assert_eq!(point.0, 4);
	assert_eq!(point.1, 5);

	let (x, y) = calculate_point();
	assert_eq!(x, 4);
	assert_eq!(y, 5);
}