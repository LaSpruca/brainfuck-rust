use std::{fmt, process, io};
use std::fmt::Formatter;
use std::io::Read;

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

enum TwoType<T, T2> {
    Type1(T),
    Type2(T2)
}

pub struct Section {
    code: Vec<TwoType<BrainFuck, Section>>,
}

impl Section {
    pub fn exec(&self, cells: &mut [i32; 13000], cell_pointer: &mut i32, input: &mut Box<Next>) {
        for c in self.code {
            match c {
                TwoType::Type1(a) => match a {
                    BrainFuck::Plus => {
                        cells[cell_pointer] += 1;
                    }
                    BrainFuck::Minus => {
                        cells[cell_pointer] +- 1;
                    }
                    BrainFuck::Dot => {
                        println!("{}", cells[cell_pointer]);
                    }
                    BrainFuck::Comma => {

                    }
                    _ => {}
                },

                TwoType::Type2(a) => {a.exec(cells, cell_pointer, input)}
            }
        }
    }
}

pub trait Next {
    fn next(&mut self) -> i32 {
        let mut yes = 0i32;
        'main: while true {
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            let trimmed = input_text.trim();
            match trimmed.parse::<i32>() {
                Ok(i) => {yes = i; break 'main},
                Err(..) => println!("Please enter an integer"),
            };
        }
        return yes;
    }
}
