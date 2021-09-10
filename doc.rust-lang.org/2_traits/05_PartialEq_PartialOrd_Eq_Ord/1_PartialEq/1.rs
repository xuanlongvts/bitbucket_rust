fn main() {
	#[derive(Debug)]
	enum BookFormat {
		Paperback,
		Hardback,
		Ebook
	}

	#[derive(Debug)]
	struct Book {
		isbn: i32,
		format: BookFormat
	}

	impl PartialEq for Book {
		fn eq(&self, other: &Self) -> bool {
			println!("self lowercase: {:?}", self);
			println!("Self upercase =====> : {:?}", other);
			self.isbn == other.isbn
		}
	}

	let b1 = Book {
		isbn: 3,
		format: BookFormat::Paperback
	};
	let b2 = Book {
		isbn: 3,
		format: BookFormat::Ebook
	};
	let b3 = Book {
		isbn: 10,
		format: BookFormat::Hardback
	};
	assert!(b1 == b2);
	assert!(b1 != b3);
}