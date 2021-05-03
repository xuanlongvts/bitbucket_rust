use std::fmt;

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait WriteAnother {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn main() {
    println!("Hello, world!");
}
