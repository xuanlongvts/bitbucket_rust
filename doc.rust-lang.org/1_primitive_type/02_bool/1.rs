fn main() {
	let bool_val = true & false | false;
	assert!(!bool_val);

	let praise_the_borrow_checker = true;
	if praise_the_borrow_checker {
		println!("oh, yeah!");
	} else {
		println!("What?");
	}

	match praise_the_borrow_checker {
		true => println!("keep praising!"),
		false => println!("you should praise!"),
	}

	assert_eq!(true as i32, 1);
	assert_eq!(false as i32, 0);

	assert_eq!(false.then(|| 0), None);
	assert_eq!(true.then(|| 0), Some(0));
	assert_eq!(true.then(|| 1), Some(1));
}