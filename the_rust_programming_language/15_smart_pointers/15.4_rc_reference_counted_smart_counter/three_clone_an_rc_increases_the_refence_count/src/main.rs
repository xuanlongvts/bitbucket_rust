use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("1. count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("2. count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, a.clone());
        println!("3. count after create c = {}", Rc::strong_count(&a));
    }
    println!(
        "Last: counter after create c go out of scope = {}",
        Rc::strong_count(&a)
    );
}
