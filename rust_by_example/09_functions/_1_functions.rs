fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
	if rhs == 0 {
		return false;
	}
	lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
	if is_divisible_by(n, 15) {
		println!("fizzbuzz");
	} else if is_divisible_by(n, 3) {
		println!("fizz");
	} else if is_divisible_by(n, 5) {
		println!("buzz");
	} else {
		println!("{}", n);
	}
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
	for i in 0..n {
		fizzbuzz(i);
	}
}

fn main() {
	fizzbuzz_to(100);
}