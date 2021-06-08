fn main() {
	// {} wil automatically replaced with any arguments.
	println!("{} days", 31);

	// Position agruments
	println!("{0}, this is {1}, {1} this is {0}", "I", "IV");

	// Named arguments.
	println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jump over");

	// Special format after a `:`  {:b} is shorthan of binary, 2 = 10, 5 = 101
	println!("{} of {:b} people know library, the other half doesn't", 1, 2);

	// You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
	println!("{number:>width$}", number=1, width=6);

	// You can pad numbers with extra zeroes. This will output "001".
	println!("{number:>0width$}", number=1, width=3);

	// Rust even checks to make sure the correct number of arguments are used.
	println!("My name is {0}, {1} {0}", "Long", "Le");
}