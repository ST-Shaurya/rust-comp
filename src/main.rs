use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod lex;
mod parser;

fn main() {
    // Create a path to desired file
    let path = Path::new("test.text");
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", display, why),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => println!("{} Read", display),
        Err(why) => panic!("Couldn't read {}: {}", display, why),
    };

    // init the lexical analyzer
    let mut lex_analyzer = lex::Lex {
        cur: 0usize,
        len: 0usize,
        lineno: 0usize,
        fstr: s,
    };

    parser::parse(&mut lex_analyzer);
}
