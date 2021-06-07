fn main() {
    let number_list1 = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list1[0];
    for number in &number_list1 {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number1 is: {}", largest);

    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = &number_list2[0];
    for number in &number_list2 {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number2 is: {}", largest);

    println!("======================================");
    // solution
    let result1 = largest_i32(&number_list1);
    println!("The largest number1_1 is: {}", result1);

    let result2 = largest_i32(&number_list2);
    println!("The largest number2_1 is: {}", result2);

    println!("======================================");
    // characters
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result3 = largest_char(&char_list);
    println!("The largest char1 is: {}", result3);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
