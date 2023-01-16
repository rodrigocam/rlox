use std::{fs, env, io};

mod lexing;
mod error;

use crate::lexing::*;

fn exec(source: &str) {
    println!("Executing lox code");
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn exec_file(file_path: &str) {
    println!("Executing file");
    let src = fs::read_to_string(file_path).unwrap();
    exec(&src);
}

fn exec_repl(){
    println!("Executing repl");
    let mut line = String::new();
    loop {
        io::stdin().read_line(&mut line).unwrap();
        exec(&line);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Wrong number of arguments: {}", args.len());
    } else if  args.len() == 2 {
        exec_file(&args[1]);
    } else {
        exec_repl();
    }
}
