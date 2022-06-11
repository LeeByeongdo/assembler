extern crate core;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Peekable;
use std::ops::Add;

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

#[derive(PartialEq, Debug)]
enum CommandType {
    ACommand,
    CCommand,
    LCommand,
    IGNORE,
}

struct Parser {
    file_path: String,
    lines: Peekable<Lines<BufReader<File>>>,
    current_command: Option<String>,
    variable_address: usize,
}

impl Parser {
    fn new(file_path: String) -> Parser {
        let file = File::open(file_path.as_str()).unwrap();
        let buf_reader = BufReader::new(file);

        Parser {
            file_path,
            lines: buf_reader.lines().peekable(),
            current_command: None,
            variable_address: 16,
        }
    }

    fn parse(&mut self) {
        self.variable_address = 16;

        let mut result = String::new();

        let mut symbol_table = SymbolTable::new();

        while self.has_more_commands() {
            self.advance();
            let command_type = self.command_type();

            match command_type {
                CommandType::ACommand | CommandType::CCommand => {
                    symbol_table.command_address_counter += 1;
                }
                CommandType::LCommand => {
                    let command = self.current_command.clone().unwrap();
                    symbol_table.table.insert(
                        String::from(&command[1..command.len() - 1]),
                        symbol_table.command_address_counter,
                    );
                }
                CommandType::IGNORE => {}
            }
        }

        let file = File::open(self.file_path.as_str()).unwrap();
        let buf_reader = BufReader::new(file);
        self.lines = buf_reader.lines().peekable();

        while self.has_more_commands() {
            self.advance();

            let command_type = self.command_type();
            match command_type {
                CommandType::ACommand => {
                    let address = usize::from_str_radix(self.symbol().as_str(), 10);
                    let addr = match address {
                        Ok(number_address) => {
                            number_address
                        }
                        Err(_) => {
                            let symbol = self.symbol();
                            let addr = if symbol_table.contains(&symbol) {
                                symbol_table.get_address(&symbol)
                            } else {
                                let variable_address = self.variable_address;
                                symbol_table.add_entry(String::from(symbol), variable_address);
                                self.variable_address += 1;
                                variable_address
                            };
                            addr
                        }
                    };

                    let formatted = format!("{addr:016b}");
                    result = result.add(formatted.as_str());
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
                _ => {}
            }

            match command_type {
                CommandType::ACommand | CommandType::CCommand => {
                    result = result.add("\n");
                }
                _ => {}
            }


        }

        fs::write("program.hack", result).unwrap();
    }

    fn has_more_commands(&mut self) -> bool {
        self.lines.peek().is_some()
    }

    fn advance(&mut self) {
        if self.has_more_commands() {
            let command = self.lines.next().unwrap().unwrap();
            let index = command.find("//");

            match index {
                None => {
                    self.current_command = Some(command);
                }
                Some(idx) => {
                    self.current_command = Some(String::from(&command[0..idx]));
                }
            }
        }
    }

    fn command_type(&self) -> CommandType {
        if let None = self.current_command {
            panic!("command error");
        }

        let command = String::from(self.current_command.as_ref().unwrap().trim());

        if command.is_empty() {
            return CommandType::IGNORE;
        }

        let first_char = command.chars().nth(0);
        if let None = first_char {
            panic!("starting line char error: {}", command);
        }

        let first_char = first_char.unwrap();

        match first_char {
            '@' => CommandType::ACommand,
            '(' => CommandType::LCommand,
            _ => {
                if command[0..2].eq("//") {
                    return CommandType::IGNORE;
                }
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

        let command = String::from(self.current_command.as_ref().unwrap().trim());
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
            command = String::from(&command[index + 1..]);
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
            "null0" => "000",
            "M" => "001",
            "D" => "010",
            "MD" => "011",
            "A" => "100",
            "AM" => "101",
            "AD" => "110",
            "AMD" => "111",
            &_ => {
                panic!("bad dest input: {}", dest)
            }
        }
    }

    pub fn jump(jump: &str) -> &str {
        let jump = jump.trim();

        match jump {
            "null" => "000",
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            &_ => {
                panic!("bad jump input")
            }
        }
    }

    pub fn comp(comp: &str) -> &str {
        let comp = comp.trim().replace(' ', "");

        match comp.as_str() {
            "0" => "0101010",
            "1" => "0111111",
            "-1" => "0111010",
            "D" => "0001100",
            "A" => "0110000",
            "!D" => "0001101",
            "!A" => "0110001",
            "-D" => "0001111",
            "-A" => "0110011",
            "D+1" | "1+D" => "0011111",
            "A+1" | "1+A" => "0110111",
            "D-1" => "0001110",
            "A-1" => "0110010",
            "D+A" | "A+D" => "0000010",
            "D-A" => "0010011",
            "A-D" => "0000111",
            "D&A" | "A&D" => "0000000",
            "D|A" | "A|D" => "0010101",
            "M" => "1110000",
            "!M" => "1110001",
            "-M" => "1110011",
            "M+1" | "1+M" => "1110111",
            "M-1" => "1110010",
            "D+M" | "M+D" => "1000010",
            "D-M" => "1010011",
            "M-D" => "1000111",
            "D&M" | "M&D" => "1000000",
            "D|M" | "M|D" => "1010101",
            &_ => {
                panic!("bad comp input: {}", comp);
            }
        }
    }
}

#[derive(Debug)]
struct SymbolTable {
    table: HashMap<String, usize>,
    command_address_counter: usize,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        let mut map = HashMap::new();
        map.insert(String::from("SP"), 0);
        map.insert(String::from("LCL"), 1);
        map.insert(String::from("ARG"), 2);
        map.insert(String::from("THIS"), 3);
        map.insert(String::from("THAT"), 4);
        map.insert(String::from("R0"), 0);
        map.insert(String::from("R1"), 1);
        map.insert(String::from("R2"), 2);
        map.insert(String::from("R3"), 3);
        map.insert(String::from("R4"), 4);
        map.insert(String::from("R5"), 5);
        map.insert(String::from("R6"), 6);
        map.insert(String::from("R7"), 7);
        map.insert(String::from("R8"), 8);
        map.insert(String::from("R9"), 9);
        map.insert(String::from("R10"), 10);
        map.insert(String::from("R11"), 11);
        map.insert(String::from("R12"), 12);
        map.insert(String::from("R13"), 13);
        map.insert(String::from("R14"), 14);
        map.insert(String::from("R15"), 15);
        map.insert(String::from("SCREEN"), 16384);
        map.insert(String::from("KBD"), 24576);

        SymbolTable {
            table: map,
            command_address_counter: 0,
        }
    }

    fn add_entry(&mut self, symbol: String, address: usize) {
        self.table.insert(symbol, address);
    }

    fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    fn get_address(&self, symbol: &str) -> usize {
        *self.table.get(symbol).unwrap()
    }
}
