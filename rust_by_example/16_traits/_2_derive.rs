// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
	fn to_centimeters(&self) -> Centimeters {
		// let &Inches(inches) = self;

		let inches = self.0;

		Centimeters(inches as f64 * 2.54)
	}
}

// `Seconds`, a tuple struct with no additional attributes
struct Second(i32);

fn main() {
	let _one_second = Second(1);


	let food = Inches(12);
	println!("One foot equals {:?}", food);

	let meter = Centimeters(100.0);
	let cmp = if food.to_centimeters() < meter {
		"small"
	} else {
		"bigger"
	};
    println!("One foot is {} than one meter.", cmp);
}
