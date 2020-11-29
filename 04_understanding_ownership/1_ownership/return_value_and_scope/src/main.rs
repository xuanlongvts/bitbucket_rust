fn main() {
    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let s2 = String::from("World");
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {}", s3);

    let s4 = String::from("Long Le");
    let (s5, len) = calculate_length(s4);
    println!("s5 = {}, len = {} ", s5, len);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(str: String) -> String {
    str
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}
