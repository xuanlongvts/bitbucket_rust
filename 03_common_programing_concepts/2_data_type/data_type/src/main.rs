fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess = {}", guess);

    // floating-point type
    let x = 2.0; // f64
    let y: f32 = 4.0; // f32

    // Operations
    let sum = 5 + 10;
    let diff = 10 - 5;
    let multi = 2 * 3;
    let divi = 6 / 2;
    let remainder = 10 % 3;

    // Boolean type
    let t = true;
    let f: bool = false;

    // Charater type
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
}
