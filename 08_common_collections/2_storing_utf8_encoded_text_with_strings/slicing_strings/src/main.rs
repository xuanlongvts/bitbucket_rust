fn main() {
    let s1 = "Здравствуйте";
    let s2 = &s1[0..4];
    println!("s2 = {}", s2);
    let s3 = &s1[0..2];
    println!("s3 = {}", s3);

    // let s4 = &s1[0..1]; error

    let s5 = "नमस्ते";
    for i in s5.chars() {
        println!("{}", i);
    }

    println!("==============");
    
    for i in s5.bytes() {
        println!("{}", i);
    }
}
