pub struct Code {}

impl Code {
    pub fn dest(dest: String) -> &'static str {
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

    pub fn jump(jump: String) -> &'static str {
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

    pub fn comp(comp: String) -> &'static str {
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

#[cfg(test)]
mod tests {
    use crate::code::Code;

    #[test]
    fn test_comp() {
        assert_eq!(Code::comp(String::from("-M")), "1110011");
        assert_eq!(Code::comp(String::from("D|M")), "1010101");
        assert_eq!(Code::comp(String::from("M|D")), "1010101");
    }

    #[test]
    fn test_dest() {
        assert_eq!(Code::dest(String::from("AD")), "110");
        assert_eq!(Code::dest(String::from("AMD")), "111");
    }

    #[test]
    fn test_jump() {
        assert_eq!(Code::jump(String::from("JEQ")), "010");
        assert_eq!(Code::jump(String::from("JGE")), "011");
    }



}
