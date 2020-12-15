fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // // note s1 has been moved here and can no longer be used
    println!("s3 = {}", s3);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let s7 = s4 + " - " + &s5 + " - " + &s6;
    println!("s7 = {}", s7);

    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");
    let s11 = format!("{}-{}-{}", s8, s9, s10);
    println!("s11 = {}", s11);
}
