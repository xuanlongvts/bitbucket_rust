fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is = {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    let arr_a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < arr_a.len() {
        println!("The value is: {}", arr_a[index]);

        index += 1;
    }

    // for
    let arr_b = [1, 3, 5, 7, 9];
    for elements in arr_b.iter() {
        println!("The value is: {}", elements);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
