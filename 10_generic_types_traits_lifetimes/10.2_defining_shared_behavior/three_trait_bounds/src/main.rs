fn largest_fn<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result1 = largest_fn(&number_list);

    println!("The largest number is: {}", result1);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result2 = largest_fn(&char_list);
    println!("The largest char is: {}", result2);
}
