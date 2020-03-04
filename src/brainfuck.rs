use std::{fmt, process, io};
use std::fmt::Formatter;

pub enum BrainFuck{
    Plus,
    Minus,
    Left,
    Right,
    Dot,
    Comma,
    LBrace,
    RBrace
}

impl BrainFuck {
    pub fn parse(source: String) -> Vec<BrainFuck>{
        // Create array to hold data
        let mut code: Vec<BrainFuck> = Vec::new();

        // Loop over each character in the source code
        for char in source.chars() {
            // Convert char to &str
            let a = char.to_string();
            let c = a.as_str();

            // Convert valid characters to brainfuck::BrainFuck
            match c {
                "+" => code.push(BrainFuck::Plus) ,
                "-" => code.push(BrainFuck::Minus),
                "<" => code.push(BrainFuck::Left),
                ">" => code.push(BrainFuck::Right),
                "." => code.push(BrainFuck::Dot),
                "," => code.push(BrainFuck::Comma),
                "[" => code.push(BrainFuck::LBrace),
                "]" => code.push(BrainFuck::RBrace),
                _ => {}
            };
        };

        // Check to make sure there is source code
        if code.len() < 1 {
            println!("Source file is empty");
            process::exit(-2);
        }

        code
    }
}

impl fmt::Display for BrainFuck {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let val = match self {
            BrainFuck::Plus => "+",
            BrainFuck::Minus => "-",
            BrainFuck::Left => "<",
            BrainFuck::Right => ">",
            BrainFuck::Dot => ".",
            BrainFuck::Comma => ",",
            BrainFuck::LBrace => "[",
            BrainFuck::RBrace => "]"
        };

        write!(f, "{}", val)
    }
}
