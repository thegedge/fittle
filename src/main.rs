#[macro_use]
extern crate nom;
extern crate byteorder;

mod data_types;
mod fields;
mod messages;
mod parser;
mod types;

use std::io::prelude::*;
use std::io;

fn main() -> parser::Result {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut bytes = Vec::new();
    let _ = handle.read_to_end(&mut bytes);

    parser::parse(&bytes)
}