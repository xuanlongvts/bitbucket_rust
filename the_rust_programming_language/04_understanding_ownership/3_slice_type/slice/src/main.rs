fn main() {
    let mut s = String::from("hello world");

    let index = first_word(&s);

    let word = second_word(&s);

    println!("index = {}, word = {}", index, word);

    s.clear();

    println!("s later = {}", s);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// &String or &str
fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
