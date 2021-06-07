fn main() {
    println!("Hello, world!");
    dangle();
}

/*
fn dangle_error() -> &String {
    let s = String::from("longlx");

    &s
}
*/

fn dangle() -> String {
    let s = String::from("longlx");

    s
}
