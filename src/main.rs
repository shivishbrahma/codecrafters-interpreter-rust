use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

fn lexical_parse(input: String) {
    let mut token_map = HashMap::new();
    token_map.insert('(', "LEFT_PAREN");
    token_map.insert(')', "RIGHT_PAREN");
    token_map.insert('{', "LEFT_BRACE");
    token_map.insert('}', "RIGHT_BRACE");
    token_map.insert(',', "COMMA");
    token_map.insert(';', "SEMICOLON");
    token_map.insert('+', "PLUS");
    token_map.insert('-', "MINUS");
    token_map.insert('*', "STAR");
    token_map.insert('.', "DOT");

    let mut lexical_output: String = String::new();

    let mut line_counter: i32 = 1;
    input.chars().for_each(|c| {
        if let Some(token) = token_map.get(&c) {
            lexical_output += &format!("\n{} {} null", token, c);
        } else if c == '\n' {
            line_counter += 1;
        } else {
            println!("[line {}] Error: Unexpected character: {}", line_counter, c);
        }
    });

    println!("{}", lexical_output.trim());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                lexical_parse(file_contents);
            }
            println!("EOF  null");
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
