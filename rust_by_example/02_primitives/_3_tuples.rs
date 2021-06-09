fn reverse(pair: (i32, bool)) -> (bool, i32) {
	let (integer, boolean) = pair;
	(boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
	// A tuple with a bunch of different types
	let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
	println!("long tuple first value: {}", long_tuple.0);
	println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
	let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

	// Tuples are printable
	println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples cannot be printed
	// let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13); // < 13 elements
    // println!("too long tuple: {:?}", too_long_tuple);

	println!("the reversed is: {:?}", reverse((1, true)));

	// To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
	println!("one element type: {:?}", (5u32,));
	println!("one element type: {}", (5u32));

	//tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

	let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
	println!("matrix: {:#?}", matrix);
}