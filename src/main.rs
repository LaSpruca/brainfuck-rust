use std::{env, fs, process};

mod brainfuck;
mod csv;

fn main() {
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
    let code = brainfuck::parse(source);
    for s in code {
        print!("{}", s);
    }

    if input.len() > 0 {
        let vals = csv::parse(input);
        println!("");
        for val in vals {
            print!("{} ", val);
        }
    }

}