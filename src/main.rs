use std::fs;

use clap::Parser;

mod lex;
mod parse;
mod ulk;

#[derive(clap::Parser, Default)]
pub struct Options {
    /// Print ast
    #[clap(long, short)]
    ast: bool,

    filename: String,
}

fn main() {
    let opts = Options::parse();
    assert!(opts.filename.ends_with(".ulk"));
    let filebody = fs::read(&opts.filename).expect("Failed to read file");
    ulk::Ulk::new(&opts, &filebody).eval();
}
