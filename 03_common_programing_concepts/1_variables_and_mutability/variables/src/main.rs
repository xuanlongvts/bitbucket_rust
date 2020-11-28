fn main() {
    let mut x = 5;
    println!("First x = {}", x);

    x = 6;
    println!("Second x = {}", x);

    const MAX_OPTION: u32 = 100_000;
    println!("const max = {}", MAX_OPTION);

    let y = 2;

    let y = y + 1;
    let y = y * 2;
    println!("shadow y = {}", y);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("length of spaces = {}", spaces);
}
