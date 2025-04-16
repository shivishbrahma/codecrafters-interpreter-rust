use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier(String),
    Keyword(String),
    StringLiteral(String),
    NumberLiteral(f64),
    Operator(String),
    Punctuation(char),
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    _lexeme: String,
    _type: TokenType,
    _line: usize,
}

impl Token {
    pub fn new(_lexeme: String, _type: TokenType, _line: usize) -> Self {
        Token {
            _lexeme,
            _type,
            _line,
        }
    }
}

fn get_reserved_keyword(keyword: &str) -> Option<&str> {
    let keywords = HashMap::from([
        ("and", "AND"),
        ("class", "CLASS"),
        ("else", "ELSE"),
        ("false", "FALSE"),
        ("for", "FOR"),
        ("fun", "FUN"),
        ("if", "IF"),
        ("nil", "NIL"),
        ("or", "OR"),
        ("print", "PRINT"),
        ("return", "RETURN"),
        ("super", "SUPER"),
        ("this", "THIS"),
        ("true", "TRUE"),
        ("var", "VAR"),
        ("while", "WHILE"),
    ]);
    return keywords.get(keyword).map(|v| &**v);
}

fn get_single_char_tokens(keyword: &str) -> Option<&str> {
    let single_char_tokens = HashMap::from([
        ("!", "BANG"),
        ("=", "EQUAL"),
        ("(", "LEFT_PAREN"),
        (")", "RIGHT_PAREN"),
        ("{", "LEFT_BRACE"),
        ("}", "RIGHT_BRACE"),
        (",", "COMMA"),
        (";", "SEMICOLON"),
        ("+", "PLUS"),
        ("-", "MINUS"),
        ("*", "STAR"),
        (".", "DOT"),
        ("/", "SLASH"),
    ]);
    single_char_tokens.get(keyword).map(|v| &**v)
}

fn scanner(file_contents: String) -> (bool, Vec<Token>) {
    let mut lexeme = String::new();
    let mut line = 1;
    let mut has_error = false;
    let mut tokens: Vec<Token> = Vec::new();

    if !file_contents.is_empty() {
        let mut chars = file_contents.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                ' ' | '\t' | '\r' => continue,
                '\n' => {
                    line += 1;
                    lexeme.clear(); // Clear any potential unterminated lexemes
                }
                '"' => {
                    lexeme.clear();
                    while let Some(next_char) = chars.next() {
                        if next_char == '"' {
                            // println!("STRING \"{}\" {}", lexeme, lexeme);
                            tokens.push(Token::new(
                                lexeme.clone(),
                                TokenType::StringLiteral(lexeme.to_string()),
                                line,
                            ));
                            lexeme.clear();
                            break;
                        } else if next_char == '\n' {
                            eprintln!("[line {}] Error: Unterminated string.", line);
                            has_error = true;
                            line += 1;
                            break;
                        } else {
                            lexeme.push(next_char);
                        }

                        // Check EOF
                        if chars.peek() == None {
                            eprintln!("[line {}] Error: Unterminated string.", line);
                            has_error = true;
                            break;
                        }
                    }
                }
                '/' => {
                    if chars.peek() == Some(&'/') {
                        chars.next(); // Consume the second '/'
                        while let Some(next_char) = chars.next() {
                            if next_char == '\n' {
                                line += 1;
                                break;
                            }
                        }
                    } else {
                        // println!("SLASH / null");
                        tokens.push(Token::new(
                            String::from(c),
                            TokenType::Operator(String::from("SLASH")),
                            line,
                        ))
                    }
                }
                '=' | '!' | '>' | '<' => {
                    let op = if chars.peek() == Some(&'=') {
                        chars.next();
                        format!("{}{}", c, '=')
                    } else {
                        c.to_string()
                    };
                    let token_type = match op.as_str() {
                        "==" => "EQUAL_EQUAL",
                        "!=" => "BANG_EQUAL",
                        ">" => "GREATER",
                        ">=" => "GREATER_EQUAL",
                        "<" => "LESS",
                        "<=" => "LESS_EQUAL",
                        _ => {
                            if let Some(keyword_type) = get_single_char_tokens(op.as_str()) {
                                keyword_type
                            } else {
                                ""
                            }
                        }
                    };
                    // println!("{} {} null", token_type, op);
                    tokens.push(Token::new(
                        op.clone(),
                        TokenType::Operator(String::from(token_type)),
                        line,
                    ));
                }
                '0'..='9' => {
                    lexeme.push(c);
                    while let Some(next_char) = chars.peek() {
                        if next_char.is_digit(10) || *next_char == '.' {
                            lexeme.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    let number = lexeme.parse::<f64>().unwrap();
                    // println!(
                    //     "NUMBER {} {}{}",
                    //     lexeme,
                    //     number,
                    //     if number.fract() == 0.0 { ".0" } else { "" }
                    // );
                    tokens.push(Token::new(
                        lexeme.clone(),
                        TokenType::NumberLiteral(number),
                        line,
                    ));
                    lexeme.clear();
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    lexeme.push(c);
                    while let Some(next_char) = chars.peek() {
                        if next_char.is_alphanumeric() || *next_char == '_' {
                            lexeme.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    if let Some(keyword_type) = get_reserved_keyword(&lexeme.as_str()) {
                        // println!("{} {} null", keyword_type, lexeme);
                        tokens.push(Token::new(
                            lexeme.clone(),
                            TokenType::Keyword(String::from(keyword_type)),
                            line,
                        ));
                    } else {
                        // println!("IDENTIFIER {} null", lexeme);
                        tokens.push(Token::new(
                            lexeme.clone(),
                            TokenType::Identifier(String::from("IDENTIFIER")),
                            line,
                        ));
                    }
                    lexeme.clear();
                }
                _ => {
                    if let Some(token_type) = get_single_char_tokens(c.to_string().as_str()) {
                        // println!("{} {} null", token_type, c);
                        tokens.push(Token::new(
                            String::from(c),
                            TokenType::Operator(String::from(token_type)),
                            line,
                        ));
                    } else {
                        eprintln!("[line {}] Error: Unexpected character: {}", line, c);
                        has_error = true;
                    }
                }
            }
        }
    }
    // println!("EOF  null");
    (has_error, tokens)
}

fn tokenize(filename: &str) -> ExitCode {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    if !file_contents.is_empty() {
        let (has_error, tokens) = scanner(file_contents);
        for token in tokens {
            match token._type {
                TokenType::NumberLiteral(num) => {
                    println!(
                        "{} {} {}{}",
                        "NUMBER",
                        token._lexeme,
                        num,
                        if num.fract() == 0.0 { ".0" } else { "" }
                    );
                }
                TokenType::StringLiteral(str) => {
                    println!("{} \"{}\" {}", "STRING", token._lexeme, str)
                }
                TokenType::Identifier(_) => {
                    println!("{} {} null", "IDENTIFIER", token._lexeme)
                }
                TokenType::Operator(op) => {
                    println!("{} {} null", op, token._lexeme)
                }
                TokenType::Keyword(keyword) => {
                    println!("{} {} null", keyword, token._lexeme)
                }
                _ => {}
            }
        }

        println!("EOF  null");
        if has_error {
            return ExitCode::from(65);
        } else {
            return ExitCode::SUCCESS;
        }
    }
    println!("EOF  null");

    ExitCode::SUCCESS
}

fn parse(filename: &str) -> ExitCode {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    let mut operand_stack: Vec<f64> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();
    if !file_contents.is_empty() {
        let (_, tokens) = scanner(file_contents);
        let mut token_iter = tokens.iter().peekable();
        while let Some(token) = token_iter.next() {
            match &token._type {
                TokenType::NumberLiteral(num) => {
                    operand_stack.push(*num);
                }
                TokenType::Operator(_op) => {
                    if !operator_stack.is_empty() {
                        let top_op = operator_stack.pop().unwrap();
                        if top_op == "*" || top_op == "/" || top_op == "-" || top_op == "+" {
                            let right = operand_stack.pop().unwrap();
                            let left = operand_stack.pop().unwrap();
                            println!(
                                "({} {}{} {}{})",
                                top_op,
                                left,
                                if left.fract() == 0.0 { ".0" } else { "" },
                                right,
                                if right.fract() == 0.0 { ".0" } else { "" }
                            );
                        }
                    }
                    operator_stack.push(token._lexeme.clone());
                }
                TokenType::Identifier(_) => {
                    println!("{}", token._lexeme);
                }
                TokenType::Keyword(_) => {
                    println!("{}", token._lexeme);
                }
                TokenType::StringLiteral(_) => todo!(),
                TokenType::Punctuation(_) => todo!(),
                TokenType::EOF => todo!(),
            }
        }

        if !operator_stack.is_empty() {
            let top_op = operator_stack.pop().unwrap();
            if top_op == "*" || top_op == "/" || top_op == "-" || top_op == "+" {
                let right = operand_stack.pop().unwrap();
                let left = operand_stack.pop().unwrap();
                println!(
                    "({} {}{} {}{})",
                    top_op,
                    left,
                    if left.fract() == 0.0 { ".0" } else { "" },
                    right,
                    if right.fract() == 0.0 { ".0" } else { "" }
                );
            }
        }
    }
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return ExitCode::FAILURE;
    }

    let command = &args[1];
    let filename = &args[2];

    let s;

    match command.as_str() {
        "tokenize" => s = tokenize(filename),
        "parse" => s = parse(&filename),
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return ExitCode::FAILURE;
        }
    }
    return s;
}
