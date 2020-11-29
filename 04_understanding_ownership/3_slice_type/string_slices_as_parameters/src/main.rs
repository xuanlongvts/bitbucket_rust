fn main() {
    let s = String::from("hello world");

    let word1 = first_word(&s);
    let word2 = first_word(&s[..]);

    println!("word 1 = {}", word1);
    println!("word 2 = {}", word2);

    let my_string_literal = "hello world";
    let word_3 = first_word(&my_string_literal[..]);
    let word_4 = first_word(my_string_literal);

    println!("word 3 = {}", word_3);
    println!("word 4 = {}", word_4);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
