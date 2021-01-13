use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn closure_err() {
    let example_closure = |x| x;
    let s = example_closure(String::from("long le"));
    println!("s = {}", s);
    // let n = example_closure(5);
    /*
        The first time we call example_closure with the String value, the compiler infers the type of x and the return type of
        the closure to be String. Those types are then locked in to the closure in example_closure,
        and we get a type error if we try to use a different type with the same closure.
    */
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    closure_err();
}
