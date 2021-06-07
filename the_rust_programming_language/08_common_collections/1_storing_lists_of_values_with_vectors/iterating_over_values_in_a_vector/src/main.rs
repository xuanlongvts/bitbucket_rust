fn main() {
    let v = vec![100, 32, 5];
    for i in &v {
        println!("i = {}", i);
    }

    let mut v2 = vec![1, 2, 3, 4, 5];
    for i in &mut v2 {
        *i += 10;
    }
    for i in v2 {
        println!("i2 = {}", i);
    }
}
