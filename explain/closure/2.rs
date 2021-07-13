fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
	f(x) + f(x)
}

fn fn_square(x: i32) -> i32 {
	x * x
}

fn compose<F, G>(x: i32, f: F, g: G) -> i32 where F: Fn(i32) -> i32, G: Fn(i32) -> i32 {
	g(f(x))
}

fn main() {
	let clo_square = |x: i32| { x * x };

	println!("twice 1: {}", twice(5, clo_square));
	println!("twice 2: {}", twice(5, fn_square));

	println!("===================");
	let result_compose = compose(5, |x: i32| x + 42, |x: i32| x * 2);
	println!("result_compose: {}", result_compose);
}