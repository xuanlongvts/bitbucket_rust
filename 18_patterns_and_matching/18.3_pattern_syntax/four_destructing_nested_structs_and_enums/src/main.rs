#![allow(unused)]

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }

    // Destructuring structs and tuples
    println!("=============");
    struct Points {
        x: i32,
        y: i32,
    }
    let ((feet, inches), Points { x, y }) = ((3, 5), Points { x: 7, y: 8 });
    println!("feet: {}, inches: {}", feet, inches);
    println!("x: {}, y: {}", x, y);
}
