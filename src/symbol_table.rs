use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<String, usize>,
    pub command_address_counter: usize,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
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

    pub fn add_entry(&mut self, symbol: String, address: usize) {
        self.table.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> usize {
        *self.table.get(symbol).unwrap()
    }
}