#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
	name: &'a str,
	age: u8
}

fn main() {
	// Printing with `{:?}` is similar to with `{}`.
	println!("{:?} months in a years", 12);
	println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");

	// `Structure` is printable!
	println!("Now {:?} will print", Structure(3));

	println!("Now {:?} will print", Deep(Structure(7)));

	let name = "Peter";
	let age = 25;
	let peter = Person {name, age};
	// Pretty print
	println!("{:#?}", peter);
}