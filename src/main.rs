extern crate core;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file_path = assembler::parse_input_file(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    assembler::run(input_file_path);

}
