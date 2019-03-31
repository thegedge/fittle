#![feature(trait_alias)]

#[macro_use]
extern crate lazy_static;

mod bits;
mod fields;
mod message;
mod parser;
mod profile;

use std::io::prelude::*;
use std::io;

fn main() -> Result<(), String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut bytes = Vec::new();
    let _ = handle.read_to_end(&mut bytes);

    match parser::parse(&bytes) {
        Ok(results) => {
            match serde_json::to_string(&results) {
                Ok(json) => Ok(println!("{}", json)),
                Err(e) => Err(e.to_string()),
            }
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}
