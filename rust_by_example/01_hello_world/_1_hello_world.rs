fn main() {
	// this is comment
	/* Another comment */

	/// Document comment, run by rustc _1_hello_world.rs to build binary, after run file binary ./_1_hello_world
	println!("Hello world");
	
	let x = 5 + /* 90 + */ 5;
	println!("is `x` = 10 or 100? {}", x);
}