#![allow(unused)]

fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s1 = data.to_string();
    println!("s1 = {}", s1);

    let s2 = "s2 int content".to_string();
    println!("s2 = {}", s2);

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // update string
    let mut s_upd = String::from("Viet nam");
    s_upd.push_str(", Binh dinh");
    println!("string update 1: {}", s_upd);

    let mut s2_upd = String::from("lo");
    s2_upd.push('l');
    println!("string update 2: {}", s2_upd);
}
