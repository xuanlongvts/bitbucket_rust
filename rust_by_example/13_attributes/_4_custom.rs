#[cfg(some_condition)]
fn conditional_function() {
	println!("condition meet!");
}

fn main() {
	conditional_function();
}

// rustc --cfg some_condition _4_custom.rs && _4_custom