use std::cmp::Ordering;

fn main() {
	let x: u32 = 0;
	let y: u32 = 1;

	assert_eq!(x < y, true);
	assert_eq!(x.lt(&y), true);

	let result1 = 1.0.partial_cmp(&2.0);
	println!("result1: {:?}", result1);
	assert_eq!(result1, Some(Ordering::Less));

	let result2 = 1.0.partial_cmp(&1.0);
	assert_eq!(result2, Some(Ordering::Equal));

	let result3 = 2.0.partial_cmp(&1.0);
	assert_eq!(result3, Some(Ordering::Greater));

	let result4 = f64::NAN.partial_cmp(&1.0);
	assert_eq!(result4, None);

	let result5 = 4.0 < 5.0;
	assert_eq!(result5, true);
	let result6 = 2.0 < 1.0;
	assert_eq!(result6, false);

	let result7 = 1.0 <= 2.0;
	assert_eq!(result7, true);
	let result8 = 2.0 <= 2.0;
	assert_eq!(result8, true);

	let result9 = 1.0 > 2.0;
	assert_eq!(result9, false);
	let result10 = 2.0 > 2.0;
	assert_eq!(result10, false);

	let result11 = 2.0 >= 1.0;
	assert_eq!(result11, true);
	let result12 = 2.0 >= 2.0;
	assert_eq!(result12, true);
}