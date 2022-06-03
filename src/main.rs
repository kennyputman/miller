use std::{
    env,
    io::{self, Write},
    process,
};

mod token;

fn main() {
    let test = token::Token {
        token: token::TokenType::Semicolon,
        lexeme: String::from(":"),
        literal: String::from(":"),
        line: 23,
    };

    println!("{:?}", test);

    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => run_prompt(),
        1 => run_file(&args[0]),
        _ => {
            println!("Usage: miller [script]");
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
    println!("{}", source)
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
