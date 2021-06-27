use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
	println!("My path is: {}", args[0]);

	// The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./_12_program_arguments aa bb cc dd
	println!("I got {:?} arguments: {:?}", args.len() - 1, &args[1..]);
}