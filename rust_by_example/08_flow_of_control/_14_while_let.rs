fn main() {
	let mut optional = Some(0);

	// way 1
	loop {
		match optional {
			Some(i) => {
				if i > 9 {
					println!("Greater than 9, quit!");
					optional = None;
				} else {
					println!("`i` is `{:?}`. Try again.", i);
					optional = Some(i + 1);
				}
			},
			// Quit the loop when the destructure fails:
			_ => {
				break;
			}
		}
	}

	println!("=======================");

	// way 2 Using while let makes this sequence much nicer:
	let mut optional_1 = Some(0);

	while let Some(i) = optional_1 {
		if i > 9 {
            println!("Greater than 9, quit!");
            optional_1 = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional_1 = Some(i + 1);
        }
	}
}