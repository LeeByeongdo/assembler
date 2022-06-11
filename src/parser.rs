use crate::code::Code;
use crate::symbol_table::SymbolTable;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Peekable;
use std::ops::Add;

#[derive(PartialEq, Debug)]
enum CommandType {
    ACommand,
    CCommand,
    LCommand,
    IGNORE,
}

pub struct Parser {
    file_path: String,
    lines: Peekable<Lines<BufReader<File>>>,
    current_command: Option<String>,
    variable_address: usize,
}

impl Parser {
    pub fn new(file_path: String) -> Parser {
        let lines = Parser::get_lines(file_path.as_str());

        Parser {
            file_path,
            lines,
            current_command: None,
            variable_address: 16,
        }
    }

    pub fn parse(&mut self) {
        let mut symbol_table = self.prepare_symbol_table();

        let result = self.parse_main(&mut symbol_table);

        fs::write("program.hack", result).unwrap();
    }

    fn reset(&mut self) {
        self.variable_address = 16;
        self.lines = Parser::get_lines(self.file_path.as_str());
        self.current_command = None;
    }

    fn get_lines(file_path: &str) -> Peekable<Lines<BufReader<File>>> {
        let file = File::open(file_path).unwrap();
        let buf_reader = BufReader::new(file);
        buf_reader.lines().peekable()
    }

    fn prepare_symbol_table(&mut self) -> SymbolTable {
        self.reset();
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
                    symbol_table.add_entry(
                        String::from(&command[1..command.len() - 1]),
                        symbol_table.command_address_counter,
                    )
                }
                CommandType::IGNORE => {}
            }
        }
        symbol_table
    }

    fn parse_main(&mut self, symbol_table: &mut SymbolTable) -> String {
        self.reset();

        let mut result = String::new();

        while self.has_more_commands() {
            self.advance();

            let command_type = self.command_type();
            match command_type {
                CommandType::ACommand => {
                    let binary = self.parse_acommand(symbol_table);
                    result = result.add(binary.as_str());
                }
                CommandType::CCommand => {
                    let binary = self.parse_ccommand();
                    result = result.add(binary.as_str());
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
        result
    }

    fn parse_ccommand(&mut self) -> String {
        let dest = Code::dest(self.dest());
        let comp = Code::comp(self.comp());
        let jump = Code::jump(self.jump());

        let binary = format!("111{}{}{}", comp, dest, jump);
        binary
    }

    fn parse_acommand(&mut self, symbol_table: &mut SymbolTable) -> String {
        let symbol = self.symbol();
        let address = usize::from_str_radix(symbol.as_str(), 10);
        let addr = match address {
            Ok(number_address) => number_address,
            Err(_) => {
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
        formatted
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
