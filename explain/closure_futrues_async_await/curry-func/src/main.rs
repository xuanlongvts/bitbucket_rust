fn curry<F>(f: F, x: i32) -> impl Fn(i32) -> i32 where F: Fn(i32, i32) -> i32 {
    move |y| f(x, y)
}

fn main() {
    let add1 = |x, y| x + y;
    let closure = curry(add1, 5);
    println!("closure(1) => {}", closure(1));
}
