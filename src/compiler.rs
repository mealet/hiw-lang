// Compiler - magic wand which converts AST to virtual machine code.

use crate::{
    lexer::Value,
    parser::{Kind, Node},
    vm::Operations,
};
use colored::Colorize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Compiler {
    program: Vec<Operations>,
    functions: HashMap<String, crate::vm::Function>,
    jump_codes: Vec<usize>,
    pub pc: i32,
}

lazy_static! {
    pub static ref OPERATIONS_MAP: HashMap<&'static str, Operations> = {
        let mut m = HashMap::new();
        m.insert("PUSH", Operations::PUSH);
        m.insert("ARR", Operations::ARR);
        m.insert("SLICE", Operations::SLICE);
        m.insert("ADD", Operations::ADD);
        m.insert("SUB", Operations::SUB);
        m.insert("DIV", Operations::DIV);
        m.insert("MULT", Operations::MULT);
        m.insert("VAR", Operations::VAR);
        m.insert("FETCH", Operations::FETCH);
        m.insert("STORE", Operations::STORE);
        m.insert("TYPE", Operations::TYPE);
        m.insert("TO_INT", Operations::TO_INT);
        m.insert("TO_STR", Operations::TO_STR);
        m.insert("LEN", Operations::LEN);
        m.insert("PRINT", Operations::PRINT);
        m.insert("INPUT", Operations::INPUT);
        m.insert("LT", Operations::LT);
        m.insert("BT", Operations::BT);
        m.insert("EQ", Operations::EQ);
        m.insert("JMP", Operations::JMP);
        m.insert("JZ", Operations::JZ);
        m.insert("JNZ", Operations::JNZ);
        m.insert("DROP", Operations::DROP);
        m.insert("POP", Operations::POP);
        m.insert("CLEAN", Operations::CLEAN);
        m.insert("HALT", Operations::HALT);
        m
    };
}

// WARNING: Compare struct with binary compiler

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteCode {
    pub program: Vec<Operations>,
    pub functions: HashMap<String, crate::vm::Function>,
    pub jump_codes: Vec<usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            program: Vec::new(),
            functions: HashMap::new(),
            jump_codes: Vec::new(),
            pc: 0,
        }
    }

    pub fn error(&self, message: &str) {
        eprintln!("{} {}", "[CompilerError]:".red(), message);
        std::process::exit(1);
    }

    fn gen(&mut self, command: Operations) {
        self.program.push(command);
        self.pc = self.pc + 1;
    }

    pub fn compile_all(&mut self, nodes: Vec<Node>) -> ByteCode {
        for n in nodes {
            self.compile(n);
        }

        self.gen(Operations::HALT);

        return ByteCode {
            program: self.program.clone(),
            functions: self.functions.clone(),
            jump_codes: self.jump_codes.clone(),
        };
    }

    pub fn compile(&mut self, node: Node) -> ByteCode {
        match node.kind {
            // Types
            Kind::VAR => {
                self.gen(Operations::FETCH);
                self.gen(Operations::ARG(node.value.unwrap()));
            }
            Kind::CONST => {
                self.gen(Operations::PUSH);
                self.gen(Operations::ARG(node.value.unwrap()));
            }
            Kind::STRING => {
                self.gen(Operations::PUSH);
                self.gen(Operations::ARG(node.value.unwrap()));
            }
            Kind::BOOL => {
                self.gen(Operations::PUSH);
                self.gen(Operations::ARG(node.value.unwrap()));
            }
            Kind::ARRAY => {
                self.gen(Operations::CLEAN);

                self.compile(*node.op1.clone().unwrap());

                self.gen(Operations::ARR);
            }

            // Operations
            Kind::ADD => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());
                self.gen(Operations::ADD);
            }
            Kind::SUB => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());
                self.gen(Operations::SUB);
            }
            Kind::MULT => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());
                self.gen(Operations::MULT);
            }
            Kind::DIV => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());
                self.gen(Operations::DIV);
            }
            Kind::SET => {
                self.compile(*node.op2.clone().unwrap());
                self.gen(Operations::STORE);
                self.gen(Operations::ARG(node.op1.clone().unwrap().value.unwrap()));
            }

            // Functions and Constructions
            Kind::PRINT => {
                self.compile(*node.op1.clone().unwrap());
                self.gen(Operations::PRINT);

                if let Some(node_2) = node.op2.clone() {
                    self.compile(*node_2);
                }
                if let Some(node_3) = node.op3.clone() {
                    self.compile(*node_3);
                }
            }
            Kind::INPUT => {
                self.gen(Operations::INPUT);
            }

            Kind::IF => {
                self.compile(*node.op1.clone().unwrap());

                self.gen(Operations::JZ);
                self.jump_codes.push(self.pc as usize);
                self.gen(Operations::ARG(Value::INT(self.pc + 3)));

                let else_adress = self.pc;

                self.gen(Operations::JMP);
                self.jump_codes.push(self.pc as usize);
                self.gen(Operations::ARG(Value::INT(0)));

                self.compile(*node.op2.clone().unwrap());

                let after_adress = self.pc;

                self.program[(else_adress + 1) as usize] =
                    Operations::ARG(Value::INT(after_adress));
            }
            Kind::IF_ELSE => {
                self.compile(*node.op1.clone().unwrap());

                self.gen(Operations::JZ);
                self.jump_codes.push(self.pc as usize);
                self.gen(Operations::ARG(Value::INT(self.pc + 3)));

                let else_jmp_adress = self.pc;

                self.gen(Operations::JMP);
                self.jump_codes.push(self.pc as usize);
                self.gen(Operations::ARG(Value::INT(0)));

                self.compile(*node.op2.clone().unwrap());

                let complete_adress = self.pc;

                self.gen(Operations::JMP);
                self.jump_codes.push(self.pc as usize);
                self.gen(Operations::ARG(Value::INT(0)));

                let else_adress = self.pc;

                self.compile(*node.op3.clone().unwrap());

                let after_adress = self.pc;

                self.program[(else_jmp_adress + 1) as usize] =
                    Operations::ARG(Value::INT(else_adress));

                self.program[(complete_adress + 1) as usize] =
                    Operations::ARG(Value::INT(after_adress));
            }
            Kind::WHILE => {
                let condition_adress = self.pc;

                self.compile(*node.op1.clone().unwrap());

                self.gen(Operations::JZ);
                self.jump_codes.push(self.pc as usize);
                self.gen(Operations::ARG(Value::INT(self.pc + 3)));

                let false_jmp_adress = self.pc;

                self.gen(Operations::JMP);
                self.jump_codes.push(self.pc as usize);
                self.gen(Operations::ARG(Value::INT(0)));

                self.compile(*node.op2.clone().unwrap());

                self.gen(Operations::JMP);
                self.jump_codes.push(self.pc as usize);
                self.gen(Operations::ARG(Value::INT(condition_adress)));

                self.program[(false_jmp_adress + 1) as usize] =
                    Operations::ARG(Value::INT(self.pc));
            }
            Kind::FUNCTION_DEFINE => {
                let function_name = node.value.unwrap();

                // getting compiled expressions

                let mut args_compiler = Compiler::new();
                let args_bytes = args_compiler
                    .compile(*node.op1.clone().unwrap())
                    .program
                    .into_iter()
                    .filter(|x| x != &Operations::FETCH)
                    .collect::<Vec<Operations>>();

                let mut program_compiler = Compiler::new();

                for func in self.functions.clone() {
                    program_compiler.functions.insert(func.0, func.1);
                }

                // Compiling

                let program_bytes = program_compiler
                    .compile(*node.op2.clone().unwrap())
                    .program
                    .into_iter()
                    .filter(|x| x != &Operations::HALT)
                    .collect::<Vec<Operations>>();

                // formatting args

                let mut formatted_args: Vec<Value> = Vec::new();

                for arg in args_bytes {
                    match arg {
                        Operations::ARG(val) => {
                            formatted_args.push(val);
                        }
                        _ => {}
                    }
                }

                // creating function object

                let function = crate::vm::Function {
                    name: function_name.clone(),
                    arguments: formatted_args,
                    program: program_bytes,
                    jump_codes: program_compiler.jump_codes,
                };

                let stringify_function_name = match function_name {
                    Value::STR(str) => str,
                    _ => "undef".to_string(),
                };

                self.functions.insert(stringify_function_name, function);
            }
            Kind::FUNCTION_CALL => {
                if let Some(Value::STR(val)) = node.value {
                    let function_name = val;

                    if self.functions.contains_key(&function_name) {
                        // Initializating function object

                        let compiler_clone = self.clone();
                        let mut function_object = compiler_clone
                            .functions
                            .get(&function_name)
                            .unwrap()
                            .clone();

                        // Implementing arguments

                        let mut compiler1 = Compiler::new();
                        let mut args_node_1 = compiler1
                            .compile(*node.op1.clone().unwrap())
                            .program
                            .into_iter()
                            .collect::<Vec<Operations>>();

                        let mut args_bytes = Vec::new();
                        args_bytes.append(&mut args_node_1);

                        // Checking for second node

                        if let Some(node_2) = node.op2.clone() {
                            let mut compiler2 = Compiler::new();
                            let mut args_node_2 = compiler2
                                .compile(*node_2)
                                .program
                                .into_iter()
                                .collect::<Vec<Operations>>();

                            args_bytes.append(&mut args_node_2);
                        }

                        // Formatting and comparing args

                        let args_length = args_bytes.len();

                        if args_length / 2 < function_object.arguments.len() {
                            if let Value::STR(func_name) = function_object.clone().name {
                                self.error(
                                    format!(
                                        "Not enough arguments for calling '{}' function!",
                                        func_name
                                    )
                                    .as_str(),
                                );
                            }
                        } else if args_length / 2 > function_object.arguments.len() {
                            if let Value::STR(func_name) = function_object.clone().name {
                                self.error(
                                    format!("Too much arguments for '{}' function!", func_name)
                                        .as_str(),
                                );
                            }
                        }

                        self.program.append(&mut args_bytes);

                        // Generating variables for arguments

                        for (_, arg) in function_object.arguments.iter().rev().enumerate() {
                            self.gen(Operations::STORE);
                            self.program.push(Operations::ARG(arg.clone()));
                        }

                        // Fixing jump codes in function

                        for _position in function_object.jump_codes.clone() {
                            if let Operations::ARG(Value::INT(_code)) =
                                function_object.program[_position]
                            {
                                let formatted_code = self.pc + args_length as i32 + 2 + _code;

                                function_object.program[_position] =
                                    Operations::ARG(Value::INT(formatted_code));
                            } else {
                                self.error("Error with formatting function codes");
                            }
                        }

                        let mut function_program = function_object.program.clone();

                        self.program.append(&mut function_program);

                        function_object.arguments.iter().for_each(|arg| {
                            self.gen(Operations::DROP);
                            self.gen(Operations::ARG(arg.clone()))
                        });
                    } else {
                        self.error(
                            format!("Function '{}' is not defined here!", &function_name).as_str(),
                        );
                    }
                }
            }

            Kind::BRACK_ENUM => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());
            }
            Kind::ARGS_ENUM => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());
            }
            Kind::SLICE => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());

                self.gen(Operations::SLICE);
            }

            Kind::OP_MACRO => {
                let mut args_compiler = Compiler::new();
                let arguments = args_compiler
                    .compile(*node.op1.clone().unwrap())
                    .program
                    .into_iter()
                    .filter(|x| x != &Operations::FETCH && x != &Operations::PUSH)
                    .collect::<Vec<Operations>>();

                for arg in arguments {
                    if let Operations::ARG(Value::STR(string_argument)) = arg {
                        if OPERATIONS_MAP.contains_key(&string_argument.as_str()) {
                            let matched_operation =
                                OPERATIONS_MAP.get(&string_argument.as_str()).unwrap();

                            if [Operations::JZ, Operations::JNZ, Operations::JMP]
                                .contains(matched_operation)
                            {
                                self.gen(matched_operation.clone());
                                self.jump_codes.push(self.pc as usize);
                            } else {
                                self.gen(matched_operation.clone());
                            }
                        } else {
                            self.gen(Operations::ARG(Value::STR(string_argument)));
                        }
                    } else {
                        self.gen(arg);
                    }
                }
            }

            Kind::FILE_IMPORT => {
                if let Some(Value::STR(_str)) = node.value {
                    // finding file
                    let _filepath = crate::filereader::search_import(_str.clone());

                    if _filepath == "FILE_NOT_FOUND_1_HIW_ERROR" {
                        self.error(format!("Import '{}' not found!", _str).as_str());
                    }

                    let _source = crate::filereader::get_code(_filepath);

                    // compiling source code

                    let _lexer = crate::lexer::Lexer::new(_source, _str);

                    let mut lexer_clone = _lexer.clone();
                    while lexer_clone.token != Some(crate::lexer::Token::EOF) {
                        lexer_clone.next_token();
                    }

                    if lexer_clone.errors.len() > 0 {
                        for err in lexer_clone.errors {
                            eprintln!("{}", err);
                        }
                        std::process::exit(1);
                    }

                    let mut _parser = crate::parser::Parser::new(_lexer);
                    let _ast = _parser.parse();

                    if _parser.errors.len() > 0 {
                        for err in _parser.errors {
                            eprintln!("{}", err);
                            std::process::exit(1);
                        }
                    }

                    let mut _compiler = crate::compiler::Compiler::new();
                    let _byte_code = _compiler.compile_all(_ast);

                    // for first copying functions to the main byte code

                    for func in _byte_code.functions {
                        self.functions.insert(func.0, func.1);
                    }

                    // next format imported program to main

                    let mut program_object = _byte_code.program;

                    for _pos in _byte_code.jump_codes {
                        if let Operations::ARG(Value::INT(_code)) = program_object[_pos] {
                            let formatted_code = self.pc + _code;
                            program_object[_pos] = Operations::ARG(Value::INT(formatted_code));
                        }

                        // soon...
                    }

                    // deleting HALT for continue the program
                    let _ = program_object.pop();

                    // now we can attach it to the current

                    self.program.append(&mut program_object);
                }
            }

            // Conditions
            Kind::LT => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());
                self.gen(Operations::LT);
            }
            Kind::BT => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());
                self.gen(Operations::BT);
            }
            Kind::EQ => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());
                self.gen(Operations::EQ);
            }

            // Etc.
            Kind::EMPTY => {
                if let Some(op1) = node.op1 {
                    self.compile(*op1);
                }
                if let Some(op2) = node.op2 {
                    self.compile(*op2);
                }
                if let Some(op3) = node.op3 {
                    self.compile(*op3);
                }
            }
            Kind::SEQ => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());
            }
            Kind::PROG | Kind::EXPR => {
                if let Some(op1) = node.op1 {
                    self.compile(*op1);
                }
                if let Some(op2) = node.op2 {
                    self.compile(*op2);
                }
                if let Some(op3) = node.op3 {
                    self.compile(*op3);
                }
            }
            _ => {}
        }

        if node.kind == Kind::PROG {
            self.gen(Operations::HALT);
        }

        ByteCode {
            program: self.program.clone(),
            functions: self.functions.clone(),
            jump_codes: self.jump_codes.clone(),
        }
    }
}
