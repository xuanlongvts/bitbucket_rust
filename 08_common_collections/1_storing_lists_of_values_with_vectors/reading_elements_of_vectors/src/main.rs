#![allow(unused)]

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        None => println!("There is no third element."),
        Some(third) => println!("The third element is {}", third),
    }

    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100'
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}
