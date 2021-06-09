enum WebEvent {
	PageLoad,
	PageUnLoad,
	KeyPress(char),
	Paste(String),
	Click {
		x: i64,
		y: i64,
	},
}

fn inspect(event: WebEvent) {
	match event {
		WebEvent::PageLoad => println!("Page loaded"),
		WebEvent::PageUnLoad => println!("Page unloaded"),
		WebEvent::KeyPress(c) => println!("Press '{}'.", c),
		WebEvent::Paste(s) => println!("Pasted \"{}\".", s),
		WebEvent::Click {x, y} => {
			println!("clicked at x={}, y={}.", x, y);
		},
	}
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
	Add,
	Subtract,
}
// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// alis by self
impl VeryVerboseEnumOfThingsToDoWithNumbers {
	fn run(&self, x: i32, y: i32) -> i32 {
		match self {
			Self::Add => x + y,
			Self::Subtract => x - y,
		}
	}
}

fn main() {
	let pressed = WebEvent::KeyPress('x');

	// `to_owned()` creates an owned `String` from a string slice.
	let pasted = WebEvent::Paste("My text".to_owned());
	let click = WebEvent::Click {x: 20, y: 30};
	let load = WebEvent::PageLoad;
	let unload = WebEvent::PageUnLoad;

	inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

	let alias_var_add = Operations::Add;
	let alias_var_sub = Operations::Subtract;

}