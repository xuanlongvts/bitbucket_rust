#[derive(Clone, Copy)]
struct Point {
	x: i32,
	y: i32
}

fn main() {
	let c = 'Q';

	let ref ref_c1 = c;
	let ref_c2 = &c;

	println!("ref_c1 == ref_c2: {}", ref_c1 == ref_c2);

	let point = Point {
		x: 0,
		y: 0
	};
    // `ref` is also valid when destructuring a struct.
	let _copy_of_x = {
		let Point { x: ref ref_to_x, y: _ } = point;

		// Return a copy of the `x` field of `point`.
		*ref_to_x
	};

    // A mutable copy of `point`
	let mut mutable_point = point;
	{
		let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;
		*mut_ref_to_y = 1;
	}

  	println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // A mutable tuple that includes a pointer
	let mut mutable_tuple = (Box::new(5u32), 3u32);
	{
	    // Destructure `mutable_tuple` to change the value of `last`.
		let (_, ref mut last) = mutable_tuple;

		*last = 20u32;
	}
	println!("tuple is: {:?}", mutable_tuple);
}