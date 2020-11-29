fn main() {
    let mut s = String::from("Hello");

    {
        let r_1 = &mut s;
        println!("r1 = {}", r_1);
    }
    let r_2 = &mut s;
    println!("r2 = {}", r_2);

    change();
}

/*
fn error_show() {
    let mut s = String::from("longlx");

    let r1 = &mut s;
    let r2 = &mut s;
}


fn error_show_two() {
    let mut s = String::from("Long lx");

    let l1 = &s;
    let l2 = &s;
    let l3 = &mut s;
    println!("{}, {}, and {}", l1, l2, l3);
}
*/

fn change() {
    let mut str_new = String::from("nong no");

    let l1 = &str_new;
    let l2 = &str_new;
    println!("l1 = {}, l2 ={}", l1, l2);

    let l3 = &mut str_new;
    println!("l3 = {}", l3);
}
