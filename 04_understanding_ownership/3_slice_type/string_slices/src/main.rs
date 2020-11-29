fn main() {
    let s = String::from("Hello world");

    let s1 = &s[0..5];
    let s2 = &s[6..11];

    println!("s1 = {}, s2 = {}", s1, s2);

    let s3 = String::from("hello");
    let s4 = &s3[0..2];
    let s5 = &s3[..2];
    println!("s4 = {}, s5 = {}", s4, s5);

    let s3_len = s3.len();
    let s6 = &s3[3..s3_len];
    let s7 = &s3[3..];
    println!("s6 = {}, s7 = {}", s6, s7);

    let s8 = &s3[0..s3_len];
    let s9 = &s3[..];
    println!("s8 = {}, s9 = {}", s8, s9);
}
