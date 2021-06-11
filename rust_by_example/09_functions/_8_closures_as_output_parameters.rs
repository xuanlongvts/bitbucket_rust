fn create_fn() -> impl Fn() {
	let text = "Fn".to_owned();

	move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
	let text = "FnMut".to_owned();

	move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
	let text = "FnOne".to_owned();

	move || println!("This is a: {}", text)
}

fn main() {
	let fn_plain = create_fn();
	let mut fn_mut = create_fnmut();
	let fn_once = create_fnonce();

	fn_plain();
	fn_mut();
	fn_once();

	println!("==========================");
	fn_plain(); // allow more than 1 time

	fn_mut(); // allow more than 1 time
	fn_mut(); // allow more than 1 time

	// error value used here after move
	// fn_once();
}