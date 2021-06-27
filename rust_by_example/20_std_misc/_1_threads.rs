use std::thread;

const NTHREADS: u32 = 10;

fn main() {
	// Make a vector to hold the children which are spawned.
	let mut children = vec![];

	for i in 0..NTHREADS {
	    // Spin up another thread
		children.push(thread::spawn(move || {
			println!("this is thread number {}", i);
		}));
	}

	// println!("children: {:?}", children);

	for child in children {
	    // Wait for the thread to finish. Returns a result.
		let _ = child.join();
	}
}