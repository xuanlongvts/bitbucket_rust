fn main() {
    // matching literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching name variables
    let xx = Some(5);
    let zz = 10;
    match xx {
        Some(50) => println!("Got 50"),
        Some(zz) => println!("Matched, z = {:?}", zz),
        _ => println!("Default case, x = {:?}", xx),
    }
    println!("at the end: xx = {:?}, zz = {:?}", xx, zz);

    // multi patterns
    let xxx = 1;
    match xxx {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges of value with ..=
    let x5 = 5;
    match x5 {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let cha = 'c';
    match cha {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
