fn bar() -> ! {
    println!("bar");
    panic!()
}

fn main() {
    println!("Hi bar: {}", bar());
}
