// Parser - hardest module in compiler (ig). It creates Binary Tree with abstract image of code

#[allow(dead_code, unused)]

type LEXER = crate::lexer::Lexer;
type VALUE = crate::lexer::Value;
type OPTION = Option<Box<Node>>;

use crate::lexer::{Token, Value};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Kind {
    VAR,
    CONST,
    STRING,
    ADD,
    SUB,
    SET,
    PRINT,
    EMPTY,
    PROG,
    EXPR,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub kind: Kind,
    pub value: Option<VALUE>,
    pub op1: OPTION,
    pub op2: OPTION,
    pub op3: OPTION,
}

impl Node {
    fn new(kind: Kind, value: Option<VALUE>, op1: OPTION, op2: OPTION, op3: OPTION) -> Self {
        Node {
            kind,
            value,
            op1,
            op2,
            op3,
        }
    }
}

pub struct Parser {
    lexer: LEXER,
}

impl Parser {
    pub fn new(lexer: LEXER) -> Self {
        Parser { lexer }
    }

    fn error(&self, message: String) {
        println!("Parser Error: {}", message);
        std::process::exit(1);
    }

    fn term(&mut self) -> Node {
        let token = self.lexer.token.clone().unwrap();

        match token {
            Token::ID => {
                let node = Node::new(
                    Kind::VAR,
                    Some(self.lexer.value.clone().unwrap()),
                    None,
                    None,
                    None,
                );
                self.lexer.next_token();
                return node;
            }
            Token::NUM => {
                let node = Node::new(
                    Kind::CONST,
                    Some(self.lexer.value.clone().unwrap()),
                    None,
                    None,
                    None,
                );
                self.lexer.next_token();
                return node;
            }
            Token::STR => {
                let mut ident = String::new();
                while self.lexer.token.unwrap() == Token::STR {
                    if let Value::STR(str_val) = self.lexer.value.clone().unwrap() {
                        ident.push_str(format!("{}", str_val).as_str());
                    }
                    self.lexer.next_token();
                }

                let node = Node::new(Kind::STRING, Some(Value::STR(ident)), None, None, None);
                self.lexer.next_token();
                return node;
            }
            _ => return self.paren_expression(),
        }
    }

    fn summa(&mut self) -> Node {
        let mut node = self.term();
        let mut kind = Kind::EMPTY;

        while self.lexer.token.clone().unwrap() == Token::PLUS
            || self.lexer.token.clone().unwrap() == Token::MINUS
        {
            match self.lexer.token.clone().unwrap() {
                Token::PLUS => kind = Kind::ADD,
                Token::MINUS => kind = Kind::SUB,
                _ => {}
            }

            self.lexer.next_token();
            node = Node::new(
                kind.clone(),
                None,
                Some(Box::new(node.clone())),
                Some(Box::new(self.term())),
                None,
            );
        }

        return node;
    }

    fn paren_expression(&mut self) -> Node {
        let token = self.lexer.token.clone().unwrap();

        // if token != Token::LPAR {
        // self.error("'(' expected!".to_string());
        // }
        self.lexer.next_token();
        let node = self.expression();
        // if self.lexer.token.unwrap() != Token::RPAR {
        // self.error("')' expected".to_string());
        // }
        self.lexer.next_token();
        return node;
    }

    fn expression(&mut self) -> Node {
        let token = self.lexer.token.clone().unwrap();

        if token != Token::ID {
            return self.summa();
        }
        let mut node = self.summa();
        if node.kind == Kind::VAR && self.lexer.token.clone().unwrap() == Token::EQUAL {
            self.lexer.next_token();
            node = Node::new(
                Kind::SET,
                None,
                Some(Box::new(node.clone())),
                Some(Box::new(self.expression())),
                None,
            );
        }

        return node;
    }

    fn statement(&mut self) -> Node {
        let mut node = Node::new(Kind::EMPTY, None, None, None, None);

        let token = self.lexer.token.clone().unwrap();

        match token {
            Token::SEMICOLON => {
                node = Node::new(Kind::EMPTY, None, None, None, None);
                self.lexer.next_token();
            }
            Token::PRINT => {
                self.lexer.next_token();
                node = Node::new(
                    Kind::PRINT,
                    None,
                    Some(Box::new(self.paren_expression())),
                    None,
                    None,
                );
            }
            _ => {
                node = Node::new(
                    Kind::EXPR,
                    None,
                    Some(Box::new(self.expression())),
                    None,
                    None,
                );
                if self.lexer.token.clone().unwrap() != Token::SEMICOLON {
                    self.error("';' expected".to_string());
                }
                self.lexer.next_token()
            }
        }

        return node;
    }

    pub fn parse(&mut self) -> Node {
        self.lexer.next_token();
        let mut program_node = Node::new(Kind::PROG, None, None, None, None);
        let mut last_node = &mut program_node;

        while self.lexer.token.clone().unwrap() != Token::EOF {
            let stmt = self.statement();
            if last_node.kind == Kind::PROG && last_node.op1.is_none() {
                last_node.op1 = Some(Box::new(stmt));
            } else {
                last_node.op2 = Some(Box::new(stmt));
                last_node = last_node.op2.as_mut().unwrap();
            }
        }

        if let Some(token) = self.lexer.token {
            if token != Token::EOF {
                self.error("Invalid statement syntax".to_string());
            }
        }

        return program_node;
    }

    // pub fn parse(&mut self) -> Node {
    //     self.lexer.next_token();
    //     let program_node = Node::new(
    //         Kind::PROG,
    //         None,
    //         Some(Box::new(self.statement())),
    //         None,
    //         None,
    //     );
    //
    //     if let Some(token) = self.lexer.token {
    //         if token != Token::EOF {
    //             self.error("Invalid statement syntax".to_string());
    //         }
    //     }
    //
    //     return program_node;
    // }
}
