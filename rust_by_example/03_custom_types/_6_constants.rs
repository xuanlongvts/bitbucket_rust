static LANGUAGE: &str = "RUST";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
	n > THRESHOLD
}

fn main() {
	let n = 6;

	println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
	println!("{} is {}", n, if is_big(n) {"big"} else {"small"});

	// cannot assign to this expression
	// THRESHOLD = 5;
}