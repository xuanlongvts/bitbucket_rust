use std::fs::File;

fn main() {
    File::open("hello.txt").unwrap();

    // File::open("hello.txt").expect("Faile to open file");
}
