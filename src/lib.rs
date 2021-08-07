use std::process::exit;
use std::{
    fs::File,
    io::{prelude::*, stdin, stdout},
    path::Path,
};

mod scanner;

/**
 * Runs a REPL
 */
pub fn run_prompt() {
    let mut rslox = RsLox::new();
    loop {
        print!("> ");
        let _ = stdout().flush();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            println!("");
            break;
        }
        rslox.run(line);
        rslox.had_error = false;
    }
}

/**
 * Reads from a file with rslox statements in it
 */
pub fn run_file(file_path: &String) -> () {
    let mut rslox = RsLox::new();
    let path = Path::new(file_path);
    let mut file = File::open(path).unwrap();
    let mut program = String::new();
    file.read_to_string(&mut program).unwrap();

    rslox.run(program);
    if rslox.had_error {
        exit(65);
    }
}

/**
 * Core program runner
 */
pub struct RsLox {
    had_error: bool,
}
impl RsLox {
    pub fn new() -> Self {
        RsLox { had_error: false }
    }

    /**
     * Evaluate a string of tokens and execute them.
     */
    fn run(&mut self, program: String) {
        let scanner = scanner::Scanner::new(program);
        let tokens: Vec<scanner::Token> = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token);
        }
    }

    /*
     * Error handling
     */

    pub fn error(&mut self, line: u32, message: String) {
        self.report(line, String::from(""), message);
    }

    fn report(&mut self, line: u32, where_at: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, where_at, message);
        self.had_error = true;
    }
}