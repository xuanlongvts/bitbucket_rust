#![allow(unused)]

use std::iter;
use std::vec::IntoIter;

// This function combines two `Vec<i32>` and returns an iterator over it.
// Look how complicated its return type is!
fn combine_vecs_explicit_return_type(v: Vec<i32>, u: Vec<i32>) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
	v.into_iter().chain(u.into_iter()).cycle()
}

// This is the exact same function, but its return type uses `impl Trait`.
// Look how much simpler it is!
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item=i32> {
	v.into_iter().chain(u.into_iter()).cycle()
}

fn make_adder_function(z: i32) -> impl Fn(i32, i32) -> i32 {
	let closure = move |x, y| {x + y + z};
	closure 
}

/*
You can also use impl Trait to return an iterator that uses map or filter closures! This makes using map and filter easier. 
Because closure types don't have names, you can't write out an explicit return type if your function returns iterators with closures. 
But with impl Trait you can do this easily:
*/
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item=i32> + 'a {
	let new_vecs = numbers.iter().filter(|x| x > &&0).map(|x| x * 2);

	new_vecs
}

fn main() {
	let v1 = vec![1, 2, 3];
	let v2 = vec![4, 5];

	let mut v3 = combine_vecs(v1, v2);
	assert_eq!(Some(1), v3.next());
	assert_eq!(Some(2), v3.next());
	assert_eq!(Some(3), v3.next());
	assert_eq!(Some(4), v3.next());
	assert_eq!(Some(5), v3.next());
	println!("Done");

	let plus_one = make_adder_function(1);
	assert_eq!(plus_one(2, 3), 6);

	let nums = vec![1, 2, 3];
	
	// double_positives(&nums).next() is always = Some(2)
	// assert_eq!(Some(2), double_positives(&nums).next());
	// assert_eq!(Some(4), double_positives(&nums).next());
	// assert_eq!(Some(6), double_positives(&nums).next());

	println!("{:?}, {:?}", double_positives(&nums).next(), double_positives(&nums).next());

	let mut result_nums = double_positives(&nums);
	assert_eq!(Some(2), result_nums.next());
	assert_eq!(Some(4), result_nums.next());
	assert_eq!(Some(6), result_nums.next());
}