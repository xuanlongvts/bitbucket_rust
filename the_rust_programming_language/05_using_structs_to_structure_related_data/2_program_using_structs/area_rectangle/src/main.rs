fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    println!(
        "The other area of the rectangle is {} square pixels. ",
        area2((width, height))
    );

    let rect3 = Rectangle {
        width: 20,
        height: 10,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect3)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
