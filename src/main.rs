extern crate pest;
#[macro_use]
extern crate pest_derive;

use parser::HtmlParser;
use parser::Rule;
use pest::Parser;
use std::fs::File;
use std::io::Read;

mod parser;

fn main() {
    let mut unparsed_file = String::new();
    File::open("test.html")
        .expect("cannot open file")
        .read_to_string(&mut unparsed_file)
        .expect("cannot read file");

    let _ = HtmlParser::parse(Rule::html_document, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
}
