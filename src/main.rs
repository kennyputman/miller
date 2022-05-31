use std::{
    env,
    io::{self, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: miller [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {
    match std::fs::read(&path) {
        Ok(bytes) => {
            println!("{:?}", std::str::from_utf8(&bytes).unwrap());
        }
        Err(e) => {
            panic!("{}", e);
        }
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
