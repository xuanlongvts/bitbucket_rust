fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    println!("First value before = {}", first);

    v.push(6);

    // error
    // println!("First value after = {}", first);
}
