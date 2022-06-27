use std::{
    env,
    io::{self, Write},
    process,
};

use miller_core::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => run_prompt(),
        1 => run_file(&args[0]),
        _ => {
            println!("Usage: miller [filepath]");
            process::exit(1);
        }
    }
}

fn run_file(path: &String) {
    match std::fs::read_to_string(path) {
        Ok(source) => {
            run(&source);
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn run(source: &String) {
    let mut scanner = Scanner::new(source.to_string());

    scanner.scan_tokens();

    for token in scanner.tokens {
        println!("{:?}", token);
    }
}

fn run_prompt() {
    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        if command == "exit" {
            break;
        } else {
            println!("Command: {}", command);
        }
    }
}
