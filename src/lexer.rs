// ⌈                  ⌉
//   hiw-lang compiler
// ⌊                  ⌋
//
// Copyright ©️ 2024, mealet.
// Project licensed under the BSD-3 License
// that can be found in LICENSE file.
//
// Project Link: https://github.com/mealet/hiw-lang

// Lexer Analyzer - thing that gives me abstract data types (tokens) from just a string.

use colored::Colorize;
#[allow(unused)]
use std::collections::HashMap;
use std::process::exit;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Token {
    // Types
    NUM,
    STR,
    ID,
    // Boolean
    TRUE,
    FALSE,
    // Operations
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    EQUAL,
    // Separators
    SEMICOLON,

    LPAR,
    RPAR,

    LBRA,
    RBRA,

    LBRACK,
    RBRACK,
    // Signs
    QUOTE,
    EXCLAM,
    QUESTM,
    DOT,
    COMMA,
    COLON,
    UNDERLINE,
    // Comparsions
    LESS,
    BIGGER,
    // Functions and Constructions
    PRINT,
    INPUT,

    IF,
    ELSE,
    WHILE,
    FOR,

    // Keywords
    DEFINE,
    USING,
    IN,

    // Macros
    OP,

    // End Of File
    EOF,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    INT(i32),
    STR(String),
    BOOL(bool),
    ARRAY(Vec<Value>),
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub source_code: String,
    pub filename: String,

    pub symbols: HashMap<char, Token>,
    pub words: HashMap<String, Token>,
    pub errors: Vec<String>,

    pub input: Vec<char>,
    pub position: usize,
    pub current_line: usize,
    pub char: char,

    pub token: Option<Token>,
    pub value: Option<Value>,

    pub is_string: bool,
    pub space_before: bool,
}

impl Lexer {
    pub fn new(input: String, filename: String) -> Self {
        let symbols = HashMap::from([
            ('(', Token::LPAR),
            (')', Token::RPAR),
            ('+', Token::PLUS),
            ('-', Token::MINUS),
            ('_', Token::UNDERLINE),
            ('*', Token::MULTIPLY),
            ('/', Token::DIVIDE),
            ('=', Token::EQUAL),
            (';', Token::SEMICOLON),
            ('"', Token::QUOTE),
            ('!', Token::EXCLAM),
            ('?', Token::QUESTM),
            (':', Token::COLON),
            ('.', Token::DOT),
            (',', Token::COMMA),
            ('<', Token::LESS),
            ('>', Token::BIGGER),
            ('{', Token::LBRA),
            ('}', Token::RBRA),
            ('[', Token::LBRACK),
            (']', Token::RBRACK),
        ]);

        let words = HashMap::from([
            ("print".to_string(), Token::PRINT),
            ("input".to_string(), Token::INPUT),
            //
            ("false".to_string(), Token::FALSE),
            ("true".to_string(), Token::TRUE),
            //
            ("if".to_string(), Token::IF),
            ("else".to_string(), Token::ELSE),
            ("while".to_string(), Token::WHILE),
            ("for".to_string(), Token::FOR),
            //
            ("using".to_string(), Token::USING),
            ("define".to_string(), Token::DEFINE),
            ("in".to_string(), Token::IN),
            //
            ("op!".to_string(), Token::OP),
        ]);

        let mut lexer = Lexer {
            symbols,
            words,
            errors: Vec::new(),
            input: input.chars().collect(),
            source_code: input,
            filename,
            position: 0,
            current_line: 1,
            char: ' ',
            token: None,
            value: None,
            is_string: false,
            space_before: false,
        };

        lexer.getc();
        lexer
    }

    fn error(&mut self, message: String) {
        let current_line_source =
            self.source_code.lines().collect::<Vec<&str>>()[self.current_line - 1];

        let error_message = format!(
            "{} {}\n{}\n{}\n {} {}",
            "error:".red(),
            message,
            format!("    |- {}", self.filename).cyan(),
            "    |".cyan(),
            format!("{}  |", self.current_line).cyan(),
            current_line_source,
        );

        self.errors.push(error_message);

        self.getc();
    }

    pub fn getc(&mut self) {
        if self.position < self.input.len() {
            self.char = self.input[self.position];
            self.position += 1;
        } else {
            self.char = '\0'
        }
    }

    pub fn next_token(&mut self) {
        (self.token, self.value) = (None, None);

        while self.token.is_none() {
            match self.char {
                '\0' => self.token = Some(Token::EOF),
                '\n' => {
                    self.current_line += 1;
                    self.getc();
                }
                _ if self.char.is_whitespace() => {
                    if self.is_string {
                        self.space_before = true;
                    }

                    self.getc();
                }
                '-' => {
                    self.getc();
                    if self.char.is_digit(10) {
                        let mut value = 0;
                        while self.char.is_digit(10) {
                            value = value * 10 + self.char.to_digit(10).unwrap() as i32;
                            self.getc();
                        }

                        value = value * -1;

                        match self.is_string {
                            true => {
                                self.token = Some(Token::STR);

                                self.value = match self.space_before {
                                    true => {
                                        self.space_before = false;
                                        Some(Value::STR(format!(" {}", value)))
                                    }
                                    false => Some(Value::STR(value.to_string())),
                                }
                            }
                            false => {
                                self.token = Some(Token::NUM);
                                self.value = Some(Value::INT(value))
                            }
                        }
                    } else {
                        self.token = Some(Token::MINUS);
                    }
                }
                _ if self.symbols.contains_key(&self.char) => {
                    let matched_token = self.symbols.get(&self.char).unwrap().clone();

                    if matched_token == Token::QUOTE {
                        self.token = Some(matched_token);
                        self.is_string = !self.is_string;
                        self.getc();
                    } else {
                        match self.is_string {
                            true => {
                                self.token = Some(Token::STR);

                                self.value = match self.space_before {
                                    true => {
                                        self.space_before = false;
                                        Some(Value::STR(format!(" {}", self.char)))
                                    }
                                    false => Some(Value::STR(self.char.to_string())),
                                }
                            }
                            false => self.token = Some(matched_token),
                        }

                        self.getc();
                    }
                }
                _ if self.char.is_digit(10) => {
                    let mut value = 0;
                    while self.char.is_digit(10) {
                        value = value * 10 + self.char.to_digit(10).unwrap() as i32;
                        self.getc();
                    }

                    match self.is_string {
                        true => {
                            self.token = Some(Token::STR);

                            self.value = match self.space_before {
                                true => {
                                    self.space_before = false;
                                    Some(Value::STR(format!(" {}", value)))
                                }
                                false => Some(Value::STR(value.to_string())),
                            }
                        }
                        false => {
                            self.token = Some(Token::NUM);
                            self.value = Some(Value::INT(value))
                        }
                    }
                }
                _ if self.char.is_alphabetic() => {
                    let allowed_chars_in_id = ['!', '_', '-', '.'];

                    let mut id = String::new();
                    while self.char.is_alphanumeric() || allowed_chars_in_id.contains(&self.char) {
                        id.push(self.char);
                        self.getc();
                    }

                    if self.words.contains_key(&id) {
                        let matched_token = Some(self.words.get(&id).unwrap().clone());

                        self.token = matched_token;
                    } else {
                        match self.is_string {
                            false => {
                                self.token = Some(Token::ID);
                                self.value = Some(Value::STR(id));
                            }
                            true => {
                                self.token = Some(Token::STR);

                                self.value = match self.space_before {
                                    true => {
                                        self.space_before = false;
                                        Some(Value::STR(format!(" {}", id)))
                                    }
                                    false => Some(Value::STR(id)),
                                }
                            }
                        }
                    }
                }
                _ => {
                    let _ = self.error(format!("Undefined symbol: {}", self.char));
                }
            }
        }
    }
}
