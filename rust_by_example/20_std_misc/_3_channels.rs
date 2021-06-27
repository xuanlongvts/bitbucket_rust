use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
	let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

	let mut children = Vec::new();

	for id in 0..NTHREADS {
	    // The sender endpoint can be copied
		let thread_tx = tx.clone();

	    // Each thread will send its id via the channel
		let child = thread::spawn(move || {
			thread_tx.send(id).unwrap();

			println!("thread {} finished: ", id);
		});
		children.push(child);
	}
    // Here, all the messages are collected
	let mut ids = Vec::with_capacity(NTHREADS as usize);
	for _ in 0..NTHREADS {
		// The `recv` method picks a message from the channel `recv` will block the current thread if there are no messages available
		ids.push(rx.recv());
	}

	// Wait for the threads to complete any remaining work
	for child in children {
		child.join().expect("oops! the child thread panicked");
	}

	// Show the order in which the messages were sent
	println!("Order message: ===> {:?}", ids);
}