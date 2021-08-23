fn main() {
	let a_1 = [1, 2, 3];
	assert_eq!(a_1.len(), 3);
	assert!(!a_1.is_empty());

	let v_1 = [10, 40, 30];
	assert_eq!(Some(&10), v_1.first());

	let w_1: &[i32] = &[];
	assert_eq!(None, w_1.first());

	let x_1 = &mut [0, 1, 2];
	if let Some(first) = x_1.first_mut() {
		*first = 5;
	}
	assert_eq!(x_1, &[5, 1, 2]);

	let x_2 = &[0, 1, 2];
	if let Some((first, eles)) = x_2.split_first() {
		assert_eq!(first, &0);
		assert_eq!(eles, &[1, 2]);
	}

	let x_3 = &mut [0, 1, 2];
	if let Some((f, eles)) = x_3.split_first_mut() {
		*f = 3;
		eles[0] = 4;
		eles[1] = 5;
	}
	assert_eq!(x_3, &[3, 4, 5]);

	let v_2 = [10, 40, 30];
	assert_eq!(Some(&40), v_2.get(1));
	assert_eq!(Some(&[10, 40][..]), v_2.get(0..2));
	assert_eq!(None, v_2.get(3));
	assert_eq!(None, v_2.get(0..4));
}