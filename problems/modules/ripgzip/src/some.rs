#![allow(unused)]
fn main() {
    use std::io;
    use std::io::prelude::*;

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let buffer = stdin.fill_buf().unwrap();

    // work with buffer
    println!("{buffer:?}");

    // ensure the bytes we worked with aren't returned again later
    let length = buffer.len();
    stdin.consume(length);
}
