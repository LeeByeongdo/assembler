mod parser;
mod symbol_table;
mod code;

extern crate core;

use crate::parser::Parser;

pub fn parse_input_file(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    Ok(String::from(&args[1]))
}

pub fn run(file: String) {
    let mut parser = Parser::new(file);

    parser.parse();
}
