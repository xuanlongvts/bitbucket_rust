fn main() {
	let x = [1, 2, 3];
	let y = x.iter().map(|v| v + 1).collect::<Vec<i32>>(); // convert to vector
	println!("y: {:?}", y);

	let mut temp = 0;
	let z = x.iter().map(|v| {
		temp += 1;
		v * temp
	}).collect::<Vec<i32>>();
	println!("z: {:?}", z);

	let arr_1 = ["Ferris", "Bueller's", "Day", "Off"];
	let arr_2 = arr_1.iter().map(|v| v.len()).collect::<Vec<usize>>();
	println!("arr_2: {:?}", arr_2);

	let x1 = [1, 2, 3];
	let x2 = [4, 5, 6];
	let x3 = x1.iter().zip(x2.iter()).map(|(x, y)| (x, y));
	for (i1, i2) in x3 {
		println!("{}, {}", i1, i2);
	}
}