use std::collections::HashSet;

/*
union: get all the unique elements in both sets.
difference: get all the elements that are in the first set but not the second.
intersection: get all the elements that are only in both sets.
symmetric_difference: get all the elements that are in one set or the other, but not both.
*/

fn main() {
	let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
	let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

	assert!(a.insert(4));
	assert!(a.contains(&4));

	// `HashSet::insert()` returns false if there was a value already present.
	// assert!(b.insert(4));
	// assert with a custom message
	// assert!(b.insert(4), "Value 4 is already in set B!");

	b.insert(5);

	// If a collection's element type implements `Debug`,
    // then the collection implements `Debug`.
    // It usually prints its elements in the format `[elem1, elem2, ...]`
	println!("A: {:?}", a);
	println!("B: {:?}", b);

	// Print [1, 2, 3, 4, 5] in arbitrary order
	println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

	// This should print [1]
	println!("Different: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // Print [2, 3, 4] in arbitrary order.
	println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

	// Print [1, 5]
	println!("Symmetric Difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());
}