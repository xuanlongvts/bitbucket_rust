extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(5));
    }
}

#[no_mangle] // no change name function call_from_c when compiler build (for C call rust's function)
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
