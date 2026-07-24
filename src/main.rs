use std::fs;

use clap::Parser;

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

#[derive(clap::Parser)]
struct Options {
    /// Print ast
    #[clap(long, short)]
    ast: bool,

    filename: String,
}

fn main() {
    let opts = Options::parse();
    assert!(opts.filename.ends_with(".ulk"));
    let filebody = fs::read(opts.filename).expect("Failed to read file");
    let u = Ulk::new(&filebody);

    if opts.ast {
        println!(
            "{}",
            u.parser
                .map(|n| n.to_string())
                .reduce(|acum, s| acum + &s)
                .unwrap_or_default()
        )
    }
}
