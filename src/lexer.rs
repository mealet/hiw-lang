// Lexer Analyzer - thing that gives me abstract data types (tokens) from just a string.

#[allow(unused)]
use std::collections::HashMap;
use std::process::exit;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Token {
    NUM,
    STR,
    ID,
    PLUS,
    MINUS,
    EQUAL,
    SEMICOLON,
    LPAR,
    RPAR,
    QUOTE,
    PRINT,
    EOF,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    INT(i32),
    STR(String),
}

pub struct Lexer {
    pub symbols: HashMap<char, Token>,
    pub words: HashMap<String, Token>,
    pub input: Vec<char>,
    pub position: usize,
    pub char: char,
    pub token: Option<Token>,
    pub value: Option<Value>,
    pub is_string: bool,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let symbols = HashMap::from([
            ('(', Token::LPAR),
            (')', Token::RPAR),
            ('+', Token::PLUS),
            ('-', Token::MINUS),
            ('=', Token::EQUAL),
            (';', Token::SEMICOLON),
            ('"', Token::QUOTE),
        ]);

        let words = HashMap::from([("print".to_string(), Token::PRINT)]);

        let mut lexer = Lexer {
            symbols,
            words,
            input: input.chars().collect(),
            position: 0,
            char: ' ',
            token: None,
            value: None,
            is_string: false,
        };

        lexer.getc();
        lexer
    }

    fn error(&self, message: String) {
        eprintln!("Lexer error: {}", message);
        exit(1);
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
                _ if self.char.is_whitespace() => self.getc(),
                _ if self.symbols.contains_key(&self.char) => {
                    let matched_token = self.symbols.get(&self.char).unwrap().clone();

                    if matched_token == Token::QUOTE {
                        self.is_string = !self.is_string;
                    }

                    self.token = Some(matched_token);
                    self.getc();
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
                            self.value = Some(Value::STR(value.to_string()))
                        }
                        false => {
                            self.token = Some(Token::NUM);
                            self.value = Some(Value::INT(value))
                        }
                    }
                }
                _ if self.char.is_alphabetic() => {
                    let mut id = String::new();
                    while self.char.is_alphanumeric() {
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
                                self.value = Some(Value::STR(id));
                            }
                        }
                    }
                }
                _ => {
                    let _ = self.error(format!("Unexpected symbol: {}", self.char));
                }
            }
        }
    }
}
