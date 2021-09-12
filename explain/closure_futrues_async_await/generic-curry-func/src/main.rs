fn receives_closure<F>(closure: F) where F: Fn(i32) -> i32 {
    let result = closure(1);
    println!("closure(1) ===> {}", result);
}

fn generic_curry<F, X, Y, Z>(f: F, x: X) -> impl Fn(Y) -> Z where F: Fn(X, Y) -> Z, X: Copy {
    // f: is function closure
    // x: is X (4)
    // y: is 1
    // Z: is result
    move |y| f(x, y)
}

fn main() {
    let add = |x, y| x + y + 2;
    let closure1 = generic_curry(add, 4);
    receives_closure(closure1);
    
    println!("===============");
    let concat = |s, t: &str| format!("{}{}", s, t);
    let closure2 = generic_curry(concat, "Hello, ");
    let result = closure2("world");
    println!("result: ==> {}", result);
}
