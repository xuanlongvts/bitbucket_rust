fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("===> {}", top);
    }

    // for loop
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let statements
    let (x, y, z) = (6, 7, 8);
    println!("x = {}, y = {}, z = {}", x, y, z);

    // let (w, i) = (7, 8, 9); // error
    let (w, i, _) = (10, 11, 12);
    println!("w = {}, i = {}", w, i);

    let (k, l, ..) = (1, 2, 3, 4, 5, 6);
    println!("k = {}, l = {}", k, l);

    // function parameters
    fn print_coordinate(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (10, 20);
    print_coordinate(&point);
}
