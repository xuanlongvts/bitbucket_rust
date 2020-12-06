#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/* multiple impl block

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax.
We’ll see a case in which multiple impl blocks are useful in Chapter 10, where we discuss generic types and traits.
*/

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold react2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold react3 ? {}", rect1.can_hold(&rect3));
}
