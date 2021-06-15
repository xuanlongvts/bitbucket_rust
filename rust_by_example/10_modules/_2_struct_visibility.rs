mod my {
	pub struct OpenBox<T> {
		pub contents: T,
	}

	#[allow(dead_code)]
	pub struct CloseBox<T> {
		contents: T
	}

	impl<T> CloseBox<T> {
		pub fn new(contents: T) -> CloseBox<T> {
			CloseBox {
				contents: contents
			}
		}
	}
}

fn main() {
	let open_box = my::OpenBox {
		contents: "Public information",
	};
	println!("The open box contains: {}", open_box.contents);

	// let closed_box = my::CloseBox {
	// 	contents: "classified information",
	// };

	let _closed_box = my::CloseBox::new("classified information");

	// and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
	// println!("The closed box contains: {}", _closed_box.contents);
}