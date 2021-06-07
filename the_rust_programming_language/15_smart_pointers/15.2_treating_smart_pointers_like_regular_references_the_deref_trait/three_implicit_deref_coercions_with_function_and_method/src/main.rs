use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello: {}", name);
}

fn main() {
    let name1 = MyBox::new(String::from("Long Le"));
    let name2 = "Nong No";
    hello(&name1);

    hello(&(*name2)[..]);
}
