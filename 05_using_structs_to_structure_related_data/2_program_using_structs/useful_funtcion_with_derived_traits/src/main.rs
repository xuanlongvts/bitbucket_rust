#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("area = {:?}", rect1);
    println!("area = {:#?}", rect1);
}
