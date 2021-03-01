fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);

    // Ignoring part of value with a Nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing a customized value")
        }
        _ => setting_value = new_setting_value,
    }
    println!("setting is {:?}", setting_value);

    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    // Ignoring an unused variable by starting its name with _
    let _x = 5;
    let y = 10;
    println!("y = {}", y);

    let s = Some(String::from("hello"));
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // println!("{:?}", s); // error value borrowed here after partial move
    if let Some(_) = s {
        println!("found a striing")
    }
    println!("{:?}", s);

    // Ignoring remaining parts of a value with ..
    struct Points {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Points { x: 1, y: 2, z: 3 };
    match origin {
        Points { x, .. } => println!("x is {}", x),
    }

    let numbers_1 = (2, 4, 6, 8, 10);
    match numbers_1 {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}
