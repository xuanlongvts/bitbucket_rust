#![allow(unused)]

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    // Dropping a Vector Drops Its Elements
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here
}
