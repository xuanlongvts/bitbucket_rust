fn main() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn take_long_type(f: Box<dyn Fn() + Send + 'static>) {}

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        Box::new(|| ())
    }

    // another way
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f_1: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type_one(f: Thunk) {}
    fn returns_long_type_one() -> Thunk {
        Box::new(|| ())
    }
}
