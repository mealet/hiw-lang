// hiw-lang compiler
// https://github.com/mealet/hiw-lang
// ----------------------------------------
// Copyright ©️ 2024, mealet.
// Project licensed under the BSD-3 License
// that can be found in LICENSE file.
// ----------------------------------------

// Parser - hardest module in compiler (ig). It creates Binary Tree with abstract image of code

#[allow(dead_code, unused)]
use colored::Colorize;

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
    INPUT,

    IF,
    IF_ELSE,
    WHILE,
    FOR,

    FUNCTION_DEFINE,
    FUNCTION_CALL,

    FILE_IMPORT,

    BRACK_ENUM,
    ARGS_ENUM,
    SLICE,

    RETURN,
    OP_MACRO,
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
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: LEXER) -> Self {
        Parser {
            lexer,
            errors: Vec::new(),
        }
    }

    fn error(&mut self, message: &str) {
        let current_line_source =
            self.lexer.source_code.lines().collect::<Vec<&str>>()[self.lexer.current_line - 1];

        let error_message = format!(
            "{} {}\n{}\n{}\n {} {}",
            "[ParserError]:".red(),
            message,
            format!("    |- {}", self.lexer.filename).cyan(),
            "    |".cyan(),
            format!("{}  |", self.lexer.current_line).cyan(),
            current_line_source,
        );

        self.errors.push(error_message);

        self.lexer.next_token();
    }

    fn critical_error(&mut self, message: &str) {
        let current_line_source =
            self.lexer.source_code.lines().collect::<Vec<&str>>()[self.lexer.current_line - 1];

        let error_message = format!(
            "{} {}\n{}\n{}\n {} {}",
            "[CriticalParserError]:".red(),
            message,
            format!("    |- {}", self.lexer.filename).cyan(),
            "    |".cyan(),
            format!("{}  |", self.lexer.current_line).cyan(),
            current_line_source,
        );

        eprintln!("{}", error_message);
        std::process::exit(1);
    }

    fn term(&mut self) -> Node {
        let token = self.lexer.token.clone().unwrap();

        match token {
            Token::ID => {
                let id_name = self.lexer.value.clone().unwrap();

                let mut node = Node::new(Kind::VAR, Some(id_name.clone()), None, None, None);
                self.lexer.next_token();

                match self.lexer.token {
                    Some(Token::LBRACK) => {
                        self.lexer.next_token();

                        node = Node::new(
                            Kind::SLICE,
                            None,
                            Some(Box::new(node.clone())),
                            Some(Box::new(self.expression())),
                            None,
                        );

                        self.lexer.next_token()
                    }
                    Some(Token::LPAR) => {
                        node = Node::new(
                            Kind::FUNCTION_CALL,
                            Some(id_name),
                            Some(Box::new(self.paren_arguments())),
                            None,
                            None,
                        );
                    }
                    Some(Token::DOT) => {
                        // Going to next token which have function name
                        self.lexer.next_token();

                        node = Node::new(
                            Kind::FUNCTION_CALL,
                            Some(self.lexer.value.clone().unwrap_or_else(|| {
                                self.error("Unexpected dot after ID");
                                Value::INT(0)
                            })),
                            Some(Box::new(node.clone())),
                            None,
                            None,
                        );

                        // Searching for '(' for function call

                        self.lexer.next_token();

                        if self.lexer.token != Some(Token::LPAR) {
                            self.error("Expected '(' for function call");
                        }

                        // Parsing other arguments

                        node.op2 = Some(Box::new(self.paren_arguments()));
                    }
                    _ => {}
                };

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

                match self.lexer.token {
                    Some(Token::LBRACK) => {
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
                    Some(Token::DOT) => {
                        // Going to next token which have function name
                        self.lexer.next_token();

                        node = Node::new(
                            Kind::FUNCTION_CALL,
                            Some(self.lexer.value.clone().unwrap_or_else(|| {
                                self.error("Unexpected dot after ID");
                                Value::INT(0)
                            })),
                            Some(Box::new(node.clone())),
                            None,
                            None,
                        );

                        // Searching for '(' for function call

                        self.lexer.next_token();

                        if self.lexer.token != Some(Token::LPAR) {
                            self.error("Expected '(' for function call");
                        }

                        // Parsing other arguments

                        node.op2 = Some(Box::new(self.paren_arguments()));
                    }
                    _ => {}
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

                let mut lexer_clone = self.lexer.clone();
                lexer_clone.next_token();

                match lexer_clone.token {
                    Some(Token::DOT) => {
                        self.lexer.next_token();

                        // Going to next token which have function name
                        self.lexer.next_token();

                        node = Node::new(
                            Kind::FUNCTION_CALL,
                            Some(self.lexer.value.clone().unwrap_or_else(|| {
                                self.error("Unexpected dot after ID");
                                Value::INT(0)
                            })),
                            Some(Box::new(node.clone())),
                            None,
                            None,
                        );

                        // Searching for '(' for function call

                        self.lexer.next_token();

                        if self.lexer.token != Some(Token::LPAR) {
                            self.error("Expected '(' for function call");
                        }

                        // Parsing other arguments

                        node.op2 = Some(Box::new(self.paren_arguments()));
                    }
                    _ => {}
                };

                return node;
            }
            Token::TRUE => {
                let mut node = Node::new(Kind::BOOL, Some(Value::BOOL(true)), None, None, None);

                self.lexer.next_token();

                match self.lexer.token {
                    Some(Token::DOT) => {
                        // Going to next token which have function name
                        self.lexer.next_token();

                        node = Node::new(
                            Kind::FUNCTION_CALL,
                            Some(self.lexer.value.clone().unwrap_or_else(|| {
                                self.error("Unexpected dot after ID");
                                Value::INT(0)
                            })),
                            Some(Box::new(node.clone())),
                            None,
                            None,
                        );

                        // Searching for '(' for function call

                        self.lexer.next_token();

                        if self.lexer.token != Some(Token::LPAR) {
                            self.error("Expected '(' for function call");
                        }

                        // Parsing other arguments

                        node.op2 = Some(Box::new(self.paren_arguments()));
                    }
                    _ => {}
                }

                return node;
            }
            Token::FALSE => {
                let mut node = Node::new(Kind::BOOL, Some(Value::BOOL(false)), None, None, None);

                self.lexer.next_token();

                match self.lexer.token {
                    Some(Token::DOT) => {
                        // Going to next token which have function name
                        self.lexer.next_token();

                        node = Node::new(
                            Kind::FUNCTION_CALL,
                            Some(self.lexer.value.clone().unwrap_or_else(|| {
                                self.error("Unexpected dot after ID");
                                Value::INT(0)
                            })),
                            Some(Box::new(node.clone())),
                            None,
                            None,
                        );

                        // Searching for '(' for function call

                        self.lexer.next_token();

                        if self.lexer.token != Some(Token::LPAR) {
                            self.error("Expected '(' for function call");
                        }

                        // Parsing other arguments

                        node.op2 = Some(Box::new(self.paren_arguments()));
                    }
                    _ => {}
                };

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

        let mut node = Node::new(Kind::EMPTY, None, None, None, None);

        match self.lexer.token {
            Some(Token::RPAR) => {
                node = Node::new(Kind::EMPTY, None, None, None, None);
                self.lexer.next_token()
            }
            Some(Token::EOF) => {
                self.error("Expected ')' to end paren block!");
            }
            _ => {
                node = self.expression();
                self.lexer.next_token();
            }
        };

        return node;
    }

    fn paren_arguments(&mut self) -> Node {
        self.lexer.next_token();

        let mut node = Node::new(Kind::EMPTY, None, None, None, None);

        if self.lexer.token == Some(Token::RPAR) {
            self.lexer.next_token();
            return node;
        }

        while self.lexer.token != Some(Token::RPAR) {
            if self.lexer.token == Some(Token::COMMA) {
                self.lexer.next_token();
            }

            match self.lexer.token {
                Some(Token::COMMA) => self.lexer.next_token(),
                Some(Token::EOF) => {
                    self.critical_error("Parser got End Of File trying to parse arguments!")
                }
                Some(Token::SEMICOLON) => {
                    self.error("Parser cannot get data in '()'");
                    self.lexer.next_token();
                    break;
                }
                _ => {}
            }

            node = Node::new(
                Kind::ARGS_ENUM,
                None,
                Some(Box::new(node.clone())),
                Some(Box::new(self.expression())),
                None,
            );
        }

        self.lexer.next_token();

        return node;
    }

    fn expression(&mut self) -> Node {
        let token = self.lexer.token.clone().unwrap();

        match token {
            Token::LBRACK | Token::INPUT => {
                return self.statement();
            }
            Token::ID => {
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
            _ => return self.test(),
        }
    }

    fn statement(&mut self) -> Node {
        let mut node = Node::new(Kind::EMPTY, None, None, None, None);

        let token = self.lexer.token.clone().unwrap();

        match token {
            Token::SEMICOLON => {
                node = Node::new(Kind::EMPTY, None, None, None, None);
                self.lexer.next_token();
            }
            //
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
            Token::INPUT => {
                self.lexer.next_token();
                node = Node::new(
                    Kind::INPUT,
                    None,
                    Some(Box::new(self.paren_expression())),
                    None,
                    None,
                );
            }
            //
            Token::IF => {
                self.lexer.next_token();
                node = Node::new(
                    Kind::IF,
                    None,
                    Some(Box::new(self.expression())),
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
                    Some(Box::new(self.expression())),
                    Some(Box::new(self.statement())),
                    None,
                );

                self.lexer.next_token();
            }
            Token::FOR => {
                self.lexer.next_token();

                if self.lexer.token != Some(Token::ID) {
                    self.error("Variable name expected after 'for' keyword");

                    while self.lexer.token != Some(Token::RBRA) {
                        self.lexer.next_token();
                    }
                }

                node = Node::new(Kind::FOR, self.lexer.value.clone(), None, None, None);

                self.lexer.next_token();

                if self.lexer.token != Some(Token::IN) {
                    self.error("Keyword 'in' expected after defining variable in 'for' cycle!");

                    while self.lexer.token != Some(Token::RBRA) {
                        self.lexer.next_token();
                    }
                };

                self.lexer.next_token();

                node.op1 = Some(Box::new(self.expression()));
                node.op2 = Some(Box::new(self.statement()));

                self.lexer.next_token();
            }
            //
            Token::DEFINE => {
                self.lexer.next_token();

                node = Node::new(
                    Kind::FUNCTION_DEFINE,
                    self.lexer.value.clone(),
                    None,
                    None,
                    None,
                );

                self.lexer.next_token();

                node.op1 = Some(Box::new(self.paren_arguments()));

                match self.lexer.token {
                    Some(Token::LBRA) => {
                        node.op2 = Some(Box::new(self.statement()));
                    }
                    _ => {
                        self.error("Expected '{' after function define");
                    }
                };
            }
            Token::OP => {
                self.lexer.next_token();

                node = Node::new(
                    Kind::OP_MACRO,
                    None,
                    Some(Box::new(self.paren_arguments())),
                    None,
                    None,
                );
            }
            Token::USING => {
                self.lexer.next_token();
                self.lexer.next_token();

                if self.lexer.token != Some(Token::STR) {
                    self.error("Importing filename should be STR!");

                    while self.lexer.token != Some(Token::SEMICOLON) {
                        self.lexer.next_token();
                    }

                    return self.statement();
                } else {
                    let path_node = self.expression();

                    node = Node::new(Kind::FILE_IMPORT, path_node.value, None, None, None);

                    self.lexer.next_token();

                    if self.lexer.token != Some(Token::SEMICOLON) {
                        self.error("Expected ';' after import module");
                    } else {
                        self.lexer.next_token();
                    }
                }
            }
            //
            Token::LBRA => {
                node = Node::new(Kind::EMPTY, None, None, None, None);
                self.lexer.next_token();

                while self.lexer.token.unwrap() != Token::RBRA {
                    if self.lexer.token == Some(Token::EOF) {
                        self.error("'}' expected for ending block!");
                    }

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
            //
            _ => {
                node = Node::new(
                    Kind::EXPR,
                    None,
                    Some(Box::new(self.expression())),
                    None,
                    None,
                );

                if self.lexer.token.clone().unwrap() != Token::SEMICOLON {
                    self.error("';' expected after expression");
                }
                self.lexer.next_token();
            }
        }

        return node;
    }

    pub fn parse(&mut self) -> Vec<Node> {
        self.lexer.next_token();

        let mut statements = Vec::new();

        while self.lexer.token.clone() != Some(Token::EOF) {
            let stmt = self.statement();
            statements.push(stmt);
        }

        if let Some(token) = self.lexer.token {
            if token != Token::EOF {
                self.error("Invalid statement syntax");
            }
        }

        return statements;
    }
}
