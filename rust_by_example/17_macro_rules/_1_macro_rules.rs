macro_rules! say_hello {
	() => {
		println!("hello world");
	};
}

fn main() {
	say_hello!();
}