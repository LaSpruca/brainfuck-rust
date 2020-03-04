use std::{env, fs, process};
use std::process::exit;

mod brainfuck;
mod csv;

fn main() {
    let start = std::time::Instant::now();

    // Getting command line arguments
    let args: Vec<String> = env::args().collect();

    // Checking to see if two few or two many parameters were provided
    if args.len() < 2 {
        // Two few arguments
        println!("Usage: bf.exe <source_file> <optional: input>");
        process::exit(0);
    } else if args.len() > 3 {
        // Two many arguments
        println!("Expected max 3 arguments, {} were supplied", args.len() - 1);
        process::exit(0);
    }

    // Checking whether user wants to use input file or on-demand input
    let mut input_csv_file = true;
    if args.len() < 3 {
        input_csv_file = false;
    }

    // Checking if files paths exist
    if !fs::metadata(&args[1]).is_ok() {
        println!("Invalid file path for source");
        process::exit(-1);
    }

    if input_csv_file {
        if !fs::metadata(&args[2]).is_ok() {
            println!("Invalid file path for inputs");
            process::exit(-1);
        }
    }

    // Reading files
    let source = fs::read_to_string(&args[1]).unwrap();
    let input = if input_csv_file {fs::read_to_string(&args[2]).unwrap()} else {String::new()};

    // Parse the source code
    let code = brainfuck::BrainFuck::parse(source);
    println!("Source Code");
    for s in &code {
        print!("{}", s);
    }
    print!("\n");

    // Parsing the input values if a file is provided
    let mut vals= Vec::new();
    if input.len() > 0 {
        vals = csv::parse(input);
        println!("Input Values");
        // Printing out all values
        for val in &vals {
            print!("{} ", val);
        }
        print!("\n");
    }

    // Creating variables for execution
    let mut cells = [0; 30000];
    let mut pointer = 0;
    let mut input_pointer = 0;
    let mut braces: Vec<usize> = Vec::new();
    let mut code_pointer = 0usize;
    let code_size = code.len().clone();

    // Loop that will run as long as the code_pointer no larger the the source code
    while code_pointer < code_size{
        // Getting current code piece
        let c = &code[code_pointer];
        // Matching current code piece
        match c {
            // +
            brainfuck::BrainFuck::Plus => {
                cells[pointer] += 1;
            },
            // -
            brainfuck::BrainFuck::Minus => {
                cells[pointer] -= 1;
            },
            // <
            brainfuck::BrainFuck::Left => {
                if pointer == 0 {
                    pointer == 29999;
                } else {
                    pointer -= 1;
                }
            },
            // >
            brainfuck::BrainFuck::Right => {
                if pointer ==29999 {
                    pointer = 0;
                } else {
                    pointer += 1;
                }
            },
            // .
            brainfuck::BrainFuck::Dot => {
                println!("{}", cells[pointer]);
            },
            // ,
            brainfuck::BrainFuck::Comma => {
                if (input_csv_file) {
                    cells[pointer] = vals.get(input_pointer).expect("Error Dumbass").clone();
                    input_pointer += 1;
                } else {
                    'lp: loop {
                        let mut input_text = String::new();
                        std::io::stdin()
                            .read_line(&mut input_text)
                            .expect("failed to read from stdin");

                        let trimmed = input_text.trim();
                        match trimmed.parse::<i32>() {
                            Ok(i) => {cells[pointer] = i; break 'lp},
                            Err(..) => println!("Please enter a number"),
                        };
                    }
                }
            },
            // [
            brainfuck::BrainFuck::LBrace => {
                braces.push(code_pointer);
            },
            // ]
            brainfuck::BrainFuck::RBrace => {
                if cells[pointer] != 0 {
                    code_pointer = braces.last().expect("Error dumbass").clone();
                } else {
                    braces.pop();
                }
            }
        }; // match c
        code_pointer += 1;
    };

    // Printing the first 15 values of the array
    println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", cells[0], cells[1], cells[2], cells[3], cells[4], cells[5],  cells[6], cells[7],
           cells[8], cells[9], cells[10], cells[11], cells[12], cells[13], cells[14],);
    println!("code_pointer {}", code_pointer);

    // Calculating execution time
    let end = std::time::Instant::now();
    let difference = end.duration_since(start);
    // Printing current time if input file was supplied
    if (input_csv_file){
        println!("Execution Time {}", difference.as_micros());
    }
}
