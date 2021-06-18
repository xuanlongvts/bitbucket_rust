struct Dropable {
	name: &'static str,
}

impl Drop for Dropable {
	fn drop(&mut self) {
		println!("> Dropping {}", self.name);
	}
}

fn main() {
	let _a = Dropable {
		name: "a"
	};

	// block A
	{
		let _b = Dropable {
			name: "b"
		};

		// block B
		{
			let _c = Dropable {
				name: "c"
			};
			let _d = Dropable {
				name: "d"
			};

			println!("Exiting block B");
		}
		println!("Just exited block B");
	}
	println!("Just exited block A");

	drop(_a);

	println!("End of the main function");

	// drop(_a); // error, _a has dropped
}