fn five() -> i32 {
    5
}

/*
The five function has no parameters and defines the type of the return value, but the body of the function is a lonely 5 with no semicolon
because it’s an expression whose value we want to return.
*/

fn main() {
    let x = five();

    let y = plus_one(10);

    println!("x = {}", x);
    println!("y = {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
