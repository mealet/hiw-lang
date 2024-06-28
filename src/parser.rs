// Parser - hardest module in compiler (ig). It creates Binary Tree with abstract image of code

#[allow(dead_code, unused)]

type LEXER = crate::lexer::Lexer;
type VALUE = crate::lexer::Value;
type OPTION = Option<Box<Node>>;

use crate::lexer::{Token, Value};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Kind {
    // Types
    VAR,
    CONST,
    STRING,
    BOOL,
    ARRAY,
    EMPTY,
    // Operations
    ADD,
    SUB,
    MULT,
    DIV,
    SET,
    // Comparsions
    LT,
    BT,
    EQ,
    // Functions and Constructions
    PRINT,
    IF,
    IF_ELSE,
    WHILE,

    BRACK_ENUM,
    SLICE,
    // Etc.
    SEQ,
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

    fn error(&self, message: &str) {
        println!("[ParseError]: {}", message);
        std::process::exit(1);
    }

    fn term(&mut self) -> Node {
        let token = self.lexer.token.clone().unwrap();

        match token {
            Token::ID => {
                let mut node = Node::new(
                    Kind::VAR,
                    Some(self.lexer.value.clone().unwrap()),
                    None,
                    None,
                    None,
                );
                self.lexer.next_token();

                if self.lexer.token == Some(Token::LBRACK) {
                    self.lexer.next_token();

                    node = Node::new(
                        Kind::SLICE,
                        None,
                        Some(Box::new(node.clone())),
                        Some(Box::new(self.expression())),
                        None,
                    );

                    self.lexer.next_token();
                }

                return node;
            }
            Token::NUM => {
                let mut node = Node::new(
                    Kind::CONST,
                    Some(self.lexer.value.clone().unwrap()),
                    None,
                    None,
                    None,
                );
                self.lexer.next_token();

                if self.lexer.token == Some(Token::LBRACK) {
                    self.lexer.next_token();

                    node = Node::new(
                        Kind::SLICE,
                        None,
                        Some(Box::new(node.clone())),
                        Some(Box::new(self.expression())),
                        None,
                    );

                    self.lexer.next_token();
                }

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

                let mut node = Node::new(Kind::STRING, Some(Value::STR(ident)), None, None, None);

                return node;
            }
            Token::TRUE => {
                let node = Node::new(Kind::BOOL, Some(Value::BOOL(true)), None, None, None);
                self.lexer.next_token();
                return node;
            }
            Token::FALSE => {
                let node = Node::new(Kind::BOOL, Some(Value::BOOL(false)), None, None, None);
                self.lexer.next_token();
                return node;
            }
            Token::COMMA => {
                self.lexer.next_token();
                return self.expression();
            }
            _ => return self.paren_expression(),
        }
    }

    fn summa(&mut self) -> Node {
        let mut node = self.term();
        let mut kind = Kind::EMPTY;

        while self.lexer.token.clone().unwrap() == Token::PLUS
            || self.lexer.token.clone().unwrap() == Token::MINUS
            || self.lexer.token.clone().unwrap() == Token::MULTIPLY
            || self.lexer.token.clone().unwrap() == Token::DIVIDE
        {
            match self.lexer.token.clone().unwrap() {
                Token::PLUS => kind = Kind::ADD,
                Token::MINUS => kind = Kind::SUB,
                Token::MULTIPLY => kind = Kind::MULT,
                Token::DIVIDE => kind = Kind::DIV,
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

    fn test(&mut self) -> Node {
        let mut node = self.summa();

        match self.lexer.token.unwrap() {
            Token::LESS => {
                self.lexer.next_token();
                node = Node::new(
                    Kind::LT,
                    None,
                    Some(Box::new(node.clone())),
                    Some(Box::new(self.summa())),
                    None,
                );
            }
            Token::BIGGER => {
                self.lexer.next_token();
                node = Node::new(
                    Kind::BT,
                    None,
                    Some(Box::new(node.clone())),
                    Some(Box::new(self.summa())),
                    None,
                );
            }
            Token::EQUAL => {
                let mut lexer_copy = self.lexer.clone();
                lexer_copy.next_token();

                if lexer_copy.token == Some(Token::EQUAL) {
                    self.lexer.next_token();
                    self.lexer.next_token();

                    node = Node::new(
                        Kind::EQ,
                        None,
                        Some(Box::new(node.clone())),
                        Some(Box::new(self.summa())),
                        None,
                    );
                }
            }
            _ => {}
        }

        return node;
    }

    fn paren_expression(&mut self) -> Node {
        self.lexer.next_token();
        let node = self.expression();
        self.lexer.next_token();

        return node;
    }

    fn expression(&mut self) -> Node {
        let token = self.lexer.token.clone().unwrap();

        if token == Token::LBRACK {
            return self.statement();
        }

        if token != Token::ID {
            return self.test();
        }
        let mut node = self.test();
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

                self.lexer.next_token();
            }
            Token::IF => {
                self.lexer.next_token();
                node = Node::new(
                    Kind::IF,
                    None,
                    Some(Box::new(self.paren_expression())),
                    None,
                    None,
                );

                node.op2 = Some(Box::new(self.statement()));

                if self.lexer.token.unwrap() == Token::ELSE {
                    node.kind = Kind::IF_ELSE;
                    self.lexer.next_token();
                    node.op3 = Some(Box::new(self.statement()));
                }

                self.lexer.next_token();
            }
            Token::WHILE => {
                self.lexer.next_token();
                node = Node::new(
                    Kind::WHILE,
                    None,
                    Some(Box::new(self.paren_expression())),
                    Some(Box::new(self.statement())),
                    None,
                );

                self.lexer.next_token();
            }
            Token::LBRA => {
                node = Node::new(Kind::EMPTY, None, None, None, None);
                self.lexer.next_token();

                while self.lexer.token.unwrap() != Token::RBRA {
                    node = Node::new(
                        Kind::SEQ,
                        None,
                        Some(Box::new(node.clone())),
                        Some(Box::new(self.statement())),
                        None,
                    )
                }

                self.lexer.next_token();

                match self.lexer.token {
                    Some(Token::SEMICOLON) => {}
                    Some(Token::ELSE) => {}
                    _ => self.error("';' expected after '}'"),
                }
            }
            Token::LBRACK => {
                node = Node::new(Kind::ARRAY, None, None, None, None);
                self.lexer.next_token();

                let mut temp_node = Node::new(Kind::EMPTY, None, None, None, None);

                while self.lexer.token.unwrap() != Token::RBRACK {
                    temp_node = Node::new(
                        Kind::BRACK_ENUM,
                        None,
                        Some(Box::new(temp_node.clone())),
                        Some(Box::new(self.expression())),
                        None,
                    );
                }

                node.op1 = Some(Box::new(temp_node));

                self.lexer.next_token();
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
                    println!("{:?}", self.lexer.token);
                    self.error("';' expected after expression");
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
                self.error("Invalid statement syntax");
            }
        }

        return program_node;
    }
}
