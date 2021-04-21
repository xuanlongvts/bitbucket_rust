fn main() {
    let mut num = 5;

    // creating raw pointers from references
    let r1 = &num as *const i32;
    let r2 = &mut num as *const i32;

    unsafe {
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
    }

    // creating a raw pointer to an arbitrary memory address
    let address = 0x012345usize;
    let r_add = address as *const i32;
    unsafe { println!("rAdd = {}", *r_add) }
}
