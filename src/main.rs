use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

fn lexical_parse(input: String) -> ExitCode {
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
    token_map.insert('=', "EQUAL");
    token_map.insert('!', "BANG");
    token_map.insert('>', "GREATER");
    token_map.insert('<', "LESS");
    token_map.insert('/', "SLASH");

    let mut chars = input.chars().peekable();
    let mut literal = String::new();
    let mut line_counter: isize = 1;
    let mut has_lexical_error = false;
    let mut is_line_comment = false;
    let mut is_string = false;

    while let Some(c) = chars.next() {
        if c == '\n' {
            // Checking unterminate string error at new line
            if is_string {
                eprintln!("[line {}] Error: Unterminated string.", line_counter);
                has_lexical_error = true;
                is_string = false;
            }

            line_counter += 1;
            // Skipping till next line
            if is_line_comment {
                is_line_comment = false;
                continue;
            }
        }

        // Skipping commented character
        if is_line_comment {
            continue;
        }

        // Handling string literal
        if is_string {
            if c == '"' {
                is_string = false;
                println!("STRING \"{}\" {}", literal, literal);
                literal = String::new();
            } else {
                literal.push(c);
            }
            continue;
        }

        match c {
            '"' => {
                is_string = true;
            }
            '\t' | '\r' | ' ' | '\n' => {}
            '=' => {
                print!("{}", token_map.get(&c).unwrap());
                match chars.peek() {
                    Some('=') => {
                        println!("_{} {}{} null", token_map.get(&'=').unwrap(), c, "=");
                        chars.next();
                    }
                    _ => {
                        println!(" {} null", c);
                    }
                }
            }
            '!' => {
                print!("{}", token_map.get(&c).unwrap());
                match chars.peek() {
                    Some('=') => {
                        println!("_{} {}{} null", token_map.get(&'=').unwrap(), c, "=");
                        chars.next();
                    }
                    _ => {
                        println!(" {} null", c);
                    }
                }
            }
            '>' => {
                print!("{}", token_map.get(&c).unwrap());
                match chars.peek() {
                    Some('=') => {
                        println!("_{} {}{} null", token_map.get(&'=').unwrap(), c, "=");
                        chars.next();
                    }
                    _ => {
                        println!(" {} null", c);
                    }
                }
            }
            '<' => {
                print!("{}", token_map.get(&c).unwrap());
                match chars.peek() {
                    Some('=') => {
                        println!("_{} {}{} null", token_map.get(&'=').unwrap(), c, "=");
                        chars.next();
                    }
                    _ => {
                        println!(" {} null", c);
                    }
                }
            }
            '/' => match chars.peek() {
                Some('/') => {
                    is_line_comment = true;
                    chars.next();
                }
                _ => {
                    println!("{} {} null", token_map.get(&c).unwrap(), c);
                }
            },
            _ => {
                if let Some(token) = token_map.get(&c) {
                    println!("{} {} null", token, c);
                } else {
                    has_lexical_error = true;
                    eprintln!("[line {}] Error: Unexpected character: {}", line_counter, c);
                }
            }
        }
    }

    // Checking unterminate string error at EOF
    if is_string {
        eprintln!("[line {}] Error: Unterminated string.", line_counter);
        has_lexical_error = true;
    }

    if has_lexical_error {
        ExitCode::from(65)
    } else {
        ExitCode::SUCCESS
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return ExitCode::FAILURE;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut s = ExitCode::SUCCESS;
            if !file_contents.is_empty() {
                s = lexical_parse(file_contents);
            }
            println!("EOF  null");
            if s != ExitCode::SUCCESS {
                return s;
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
