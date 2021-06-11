/*
As opposed to all the other types, this one cannot be instantiated, because the set of all possible values this type can have is empty. 
Note that, it is different from the () type, which has exactly one possible value.
*/
#[allow(dead_code)]
fn foo() -> ! {
	panic!("This call never returns.");
}

fn some_fn() {
	()
}

fn sum_odd_numbers(up_to: u32) -> u32 {
	let mut acc = 0;
	for i in 0..up_to {
		let addition: u32 = match i % 2 == 1 {
			true => i,
			false => continue,
		};
		acc += addition;
	}
	acc
}

fn main() {
	let a: () = some_fn();
	println!("a: {:?}", a);
    println!("This function returns and you can see this line.");

	// let x: ! = panic!("This call never returns.");
	// println!("You will never see this line!");

	println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}