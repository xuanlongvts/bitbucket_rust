fn main() {
    unsafe fn dangerous() {
        println!("dangerous function");
    }

    unsafe {
        dangerous();
    }
}
