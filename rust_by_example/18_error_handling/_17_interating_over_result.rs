fn main() {
	let strings_1 = vec!["tofu", "93", "18"];
	// map()
	let numbers_1: Vec<_> = strings_1.into_iter().map(|s| s.parse::<i32>()).collect();
	println!("map, Result: {:?}", numbers_1);

	// filter_map()
	let strings_2 = vec!["tofu", "93", "18"];
	let numbers_2: Vec<_> = strings_2.into_iter().filter_map(|v| v.parse::<i32>().ok()).collect();
	println!("filter_map, Result: {:?}", numbers_2);

	// Fail the entire operation with collect()
	// Result implements FromIter so that a vector of results (Vec<Result<T, E>>) can be turned into a result with a vector (Result<Vec<T>, E>). 
	// Once an Result::Err is found, the iteration will terminate.
	// This same technique can be used with Option.
	let strings_3 = vec!["tofu", "93", "18"];
	let numbers_3: Result<Vec<_>, _> = strings_3.into_iter().map(|v| v.parse::<i32>()).collect();
	println!("Fail collect: {:?}", numbers_3);

	// Collect all valid values and failures with partition()
	let strings_4 = vec!["tofu", "93", "18"];
	let (numbers_4, errors_4): (Vec<_>, Vec<_>) = strings_4.into_iter().map(|v| v.parse::<i32>()).partition(Result::is_ok);
	println!("partition ==> Numbers: {:?}", numbers_4);
    println!("partition ==> Errors: {:?}", errors_4);

	println!("================================");
	let num_5: Vec<_> = numbers_4.into_iter().map(Result::unwrap).collect();
	println!("num_5: {:?}", num_5);
	let err_5: Vec<_> = errors_4.into_iter().map(Result::unwrap).collect();
	println!("err_5: {:?}", err_5);
}