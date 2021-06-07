fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // assert_eq!(5, y); error

    let a = 10;
    let b = Box::new(a);
    assert_eq!(10, a);
    assert_eq!(10, *b);

    // assert_eq!(10, b); error
}
