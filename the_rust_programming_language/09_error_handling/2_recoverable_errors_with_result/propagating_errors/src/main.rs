#![allow(unused)]

use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // A Shortcut for Propagating Errors: the ? Operator
    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();

        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // shorten
    fn read_username_from_file_3() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
}
