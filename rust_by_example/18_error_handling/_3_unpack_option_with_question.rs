fn next_birthday(current_age: Option<u8>) -> Option<String> {
	// If `current_age` is `None`, this returns `None`.
	// If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
	let next_age: u8 = current_age?;
	Some(
		format!("Next year I will be {}", next_age)
	)
}

#[derive(Clone, Copy)]
struct PhoneNumber {
	area_code: Option<u8>,
	number: u32,
}

#[derive(Clone, Copy)]
struct Job {
	phone_number: Option<PhoneNumber>,
}

struct Person {
	job: Option<Job>
}

impl Person {
	// Gets the area code of the phone number of the person's job, if it exists.
	fn work_phone_area_code(&self) -> Option<u8> {
		// This would need many nested `match` statements without the `?` operator.
        // It would take a lot more code - try writing it yourself and see which is easier.
		self.job?.phone_number?.area_code
	}
}

fn main() {
	println!("==>> {:?}", next_birthday(Some(10)));
	println!("==>>> {:?}", next_birthday(None));


	println!("==================");
	let p1 = Person {
		job: Some(Job {
			phone_number: Some(PhoneNumber {
				area_code: Some(61),
				number: 439222222,
			})
		})
	};
	assert_eq!(p1.work_phone_area_code(), Some(61));

	// let p2 = Person {
	// 	job: Some(Job {
	// 		phone: Some(1111),
	// 	})
	// };
	// p2.work_phone_area_code();
}