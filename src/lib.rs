extern crate core;

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Peekable;
use std::ops::{Add};

pub fn parse_input_file(args: &[String]) -> Result<&str, &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    Ok(&args[1])
}

pub fn run(file: &str) {
    let file = File::open(file).unwrap();
    let lines = BufReader::new(file).lines();
    let mut peekable = lines.peekable();

    let mut parser = Parser {
        lines: peekable,
        current_command: None,
    };

    parser.parse();
}

#[derive(PartialEq, Debug)]
enum CommandType {
    ACommand,
    CCommand,
    LCommand,
}

struct Parser {
    pub lines: Peekable<Lines<BufReader<File>>>,
    pub current_command: Option<String>,
}

impl Parser {
    fn parse(&mut self) {
        let mut result = String::new();

        while self.has_more_commands() {
            self.advance();

            let command_type = self.command_type();

            match command_type {
                CommandType::ACommand => {
                    let address = i16::from_str_radix(self.symbol().as_str(), 10).unwrap();
                    let f = format!("{address:016b}");
                    result = result.add(f.as_str());
                }
                CommandType::CCommand => {
                    let dest = self.dest();
                    let dest = Code::dest(dest.as_str());

                    let comp = self.comp();
                    let comp = Code::comp(comp.as_str());

                    let jump = self.jump();
                    let jump = Code::jump(jump.as_str());

                    let f = format!("111{}{}{}", comp, dest, jump);
                    result = result.add(f.as_str());
                }
                CommandType::LCommand => {}
            }

            result = result.add("\n");
        }

        fs::write("program.hack", result).unwrap();
    }

    fn has_more_commands(&mut self) -> bool {
        self.lines.peek().is_some()
    }

    fn advance(&mut self) {
        if self.has_more_commands() {
            let command = self.lines.next().unwrap().unwrap();
            self.current_command = Some(command);
        }
    }

    fn command_type(&self) -> CommandType {
        if let None = self.current_command {
            panic!("command error");
        }

        let command = String::from(self.current_command.as_ref().unwrap());

        let first_char = command.chars().nth(0);
        if let None = first_char {
            panic!("starting line char error");
        }

        let first_char = first_char.unwrap();

        match first_char {
            '@' => {
                CommandType::ACommand
            }
            '(' => {
                CommandType::LCommand
            }
            _ => {
                CommandType::CCommand
            }
        }
    }

    fn symbol(&self) -> String {
        if self.command_type() != CommandType::ACommand {
            panic!("symbol method should called when command type is A");
        }

        if let None = self.current_command {
            panic!("command error");
        }

        let command = String::from(self.current_command.as_ref().unwrap());
        String::from(command.get(1..).unwrap())
    }

    fn dest(&self) -> String {
        if self.command_type() != CommandType::CCommand {
            panic!("dest method should called when command type is C");
        }

        let command = String::from(self.current_command.as_ref().unwrap());

        if !command.contains('=') {
            return String::from("null0");
        }

        let dest = command.split('=').next().unwrap().trim();

        String::from(dest)
    }

    fn comp(&self) -> String {
        if self.command_type() != CommandType::CCommand {
            panic!("symbol method should called when command type is C");
        }

        let mut command = String::from(self.current_command.as_ref().unwrap());

        if command.contains('=') {
            let index = command.chars().position(|c| c == '=').unwrap();
            command = String::from(&command[index+1..]);
        }

        if command.contains(';') {
            let index = command.chars().position(|c| c == ';').unwrap();
            command = String::from(&command[..index]);
        }

        String::from(command)
    }

    fn jump(&self) -> String {
        if self.command_type() != CommandType::CCommand {
            panic!("symbol method should called when command type is C");
        }

        let command = String::from(self.current_command.as_ref().unwrap());

        if !command.contains(';') {
            return String::from("null");
        }

        let jump = command.split(';').last().unwrap().trim();

        String::from(jump)
    }
}

struct Code {}

impl Code {
    pub fn dest(dest: &str) -> &str {
        let dest = dest.trim();

        match dest {
            "null0" => {
                "000"
            }
            "M" => {
                "001"
            }
            "D" => {
                "010"
            }
            "MD" => {
                "011"
            }"A" => {
                "100"
            }"AM" => {
                "101"
            }
            "AD" => {
                "110"
            }
            "AMD" => {
                "111"
            }
            &_ => {
                panic!("bad dest input: {}", dest)
            }
        }
    }

    pub fn jump(jump: &str) -> &str {
        let jump = jump.trim();

        match jump {
            "null" => {
                "000"
            }
            "JGT" => {
                "001"
            }
            "JEQ" => {
                "010"
            }
            "JGE" => {
                "011"
            }"JLT" => {
                "100"
            }"JNE" => {
                "101"
            }
            "JLE" => {
                "110"
            }
            "JMP" => {
                "111"
            }
            &_ => {
                panic!("bad jump input")
            }
        }
    }

    pub fn comp(comp: &str) -> &str {
        let comp = comp.trim().replace(' ', "");

        match comp.as_str() {
            "0" => {
                "0101010"
            }
            "1" => {
                "0111111"
            }
            "-1" => {
                "0111010"
            }
            "D" => {
                "0001100"
            }"A" => {
                "0110000"
            }"!D" => {
                "0001101"
            }
            "!A" => {
                "0110001"
            }
            "-D" => {
                "0001111"
            }
            "-A" => {
                "0110011"
            }
            "D+1" | "1+D" => {
                "0011111"
            }
            "A+1" | "1+A" => {
                "0110111"
            }
            "D-1" => {
                "0001110"
            }
            "A-1" => {
                "0110010"
            }
            "D+A" | "A+D" => {
                "0000010"
            }
            "D-A" => {
                "0010011"
            }
            "A-D" => {
                "0000111"
            }
            "D&A" | "A&D" => {
                "0000000"
            }
            "D|A" | "A|D" => {
                "0010101"
            }
            "M" => {
                "1110000"
            }
            "!M" => {
                "1110001"
            }
            "-M" => {
                "1110011"
            }
            "M+1" | "1+M" => {
                "1110111"
            }
            "M-1" => {
                "1110010"
            }
            "D+M" | "M+D" => {
                "1000010"
            }
            "D-M" => {
                "1010011"
            }
            "M-D" => {
                "10000111"
            }
            "D&M" | "M&D" => {
                "1000000"
            }
            "D|M" | "M|D" => {
                "1010101"
            }
            &_ => {
                panic!("bad comp input")
            }
        }
    }
}