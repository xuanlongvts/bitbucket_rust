#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
	author: &'static str,
	title: &'static str,
	year: u32,
}

// this function takes a reference to a book
fn borrow_book(book: &Book) {
	println!("I immutable borrowed {} - {} edition", book.author, book.year);
}

// This function takes a reference to a mutable book and changes `year` to 2014
fn new_edition(book: &mut Book) {
	book.year = 2021;
	println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
	let immutablebook = Book {
		author: "Long Le",
		title: "Talk is cheap, show me your code",
		year: 2020,
	};

    // Create a mutable copy of `immutabook` and call it `mutabook`
	let mut mutabook = immutablebook;

	// Immutably borrow an immutable object
	borrow_book(&immutablebook);

    // Immutably borrow a mutable object
	borrow_book(&mutabook);

    // Borrow a mutable object as mutable
	new_edition(&mut mutabook);

    // Error! Cannot borrow an immutable object as mutable
	// new_edition(&mut immutablebook);
}