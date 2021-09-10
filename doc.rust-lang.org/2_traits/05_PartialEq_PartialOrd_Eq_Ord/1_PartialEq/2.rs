
// How can I compare two different types?
fn main() {
	#[derive(PartialEq)]
	enum Bookformat {
		Paperback,
		Hardback,
		Ebook
	}

	struct Book {
		isbn: i32,
		format: Bookformat
	}

	// Implement <Book> == <BookFormat> comparisons
	impl PartialEq<Bookformat> for Book {
		fn eq(&self, other: &Bookformat) -> bool {
			self.format == *other
		}
	}

	// Implement <BookFormat> == <Book> comparisons
	impl PartialEq<Book> for Bookformat {
		fn eq(&self, other: &Book) -> bool {
			*self == other.format
		}
	}

	let b1 = Book {
		isbn: 3,
		format: Bookformat::Paperback
	};
	assert!(b1 == Bookformat::Paperback);
	assert!(Bookformat::Ebook != b1);

	/* ============ */
	let x: u32 = 0;
	let y: u32 = 1;
	assert_eq!(x == y, false);
	assert_eq!(x.eq(&y), false);
}