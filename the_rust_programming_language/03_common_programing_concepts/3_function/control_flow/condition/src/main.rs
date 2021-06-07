fn main() {
    let x = 5;
    if x < 5 {
        println!("x less than 5");
    } else {
        println!("x greater than 5");
    }

    // Error
    // if x {
    //     println!("x is bool")
    // }
    /*
    Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean.
    You must be explicit and always provide if with a Boolean as its condition
    */

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // error because not the same type return
    println!("number = {}", number);
}
