fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2 = {}", s2);
    // println!("s2 = {}", s1); // error, Rust prevents you from using the invalidated reference

    let s3 = String::from("World");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive.
    // It’s a visual indicator that something different is going on.

    // Ownership and Functions
    let str = String::from("long le");
    takes_ownership(str);

    let x = 5;
    makes_copy(x);
    println!("x + 1 = {}", x + 1);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
