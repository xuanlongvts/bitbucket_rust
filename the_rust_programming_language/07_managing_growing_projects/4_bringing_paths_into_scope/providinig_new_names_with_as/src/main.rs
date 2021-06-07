#![allow(unused)]

fn main() {
    use std::fmt;
    use std::io::Result as IoResult;

    fn function1() -> fmt::Result {
        Ok(())
    }

    fn function2() -> IoResult<()> {
        Ok(())
    }

    println!("Hello, world!");
}
