// This crate is a library
#![crate_type = "lib"]

// This library is name "rary.rs"
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

fn main() {
	public_function();

	private_function();
}

// rustc _2_crates.rs
// ls lib*