fn main() {
	/* ------------ way 1 */
	let v = vec![1, 2, 3];
	println!("v: {:?}", v);
	let mut iter = v.into_iter();

	println!("iter: {:#?}", iter);
	assert_eq!(Some(1), iter.next());
	assert_eq!(Some(2), iter.next());
	assert_eq!(Some(3), iter.next());
	assert_eq!(None, iter.next());

	println!("===========================");
	/* ------------ way 2 */
	#[derive(Debug)]
	struct MyCollection(Vec<i32>);

	impl MyCollection {
		fn new() -> MyCollection {
			MyCollection(Vec::new())
		}

		fn add(&mut self, elem: i32) {
			println!("self: {:?}", self);
			self.0.push(elem);
		} 
	}
	// and we'll implement IntoIterator
	impl IntoIterator for MyCollection {
		type Item = i32;
		type IntoIter = std::vec::IntoIter<Self::Item>;

		fn into_iter(self) -> Self::IntoIter {
			self.0.into_iter()
		}
	}

	let mut c1 = MyCollection::new();
	c1.add(0);
	c1.add(1);
	c1.add(2);

	for (i, n) in c1.into_iter().enumerate() {
		assert_eq!(i as i32, n);
	}
}