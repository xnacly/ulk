use std::{env::args, fs};

mod lex;
mod parse;

struct Ulk<'ulk> {
    parser: parse::Parser<'ulk>,
}

impl<'ulk> Ulk<'ulk> {
    pub fn new(src: &'ulk [u8]) -> Ulk<'ulk> {
        Self {
            parser: parse::Parser::new(src),
        }
    }
}

fn main() {
    let filename = args()
        .last()
        .expect("No filename provided, last arg must be filename");
    assert!(filename.ends_with(".ulk"));
    let filebody = fs::read(filename).expect("Failed to read file");
    let u = Ulk::new(&filebody);
    println!(
        "{}",
        u.parser
            .map(|n| n.to_string())
            .reduce(|acum, s| acum + &s)
            .unwrap_or_default()
    )
}
