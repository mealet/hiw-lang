// Compiler - magic wand which converts AST to virtual machine code.

use crate::{
    lexer::Value,
    parser::{Kind, Node},
    vm::Operations,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Compiler {
    program: Vec<Operations>,
    functions: HashMap<String, crate::vm::Function>,
    pc: i32,
}

// WARNING: Compare struct with binary compiler

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteCode {
    pub program: Vec<Operations>,
    pub functions: HashMap<String, crate::vm::Function>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            program: Vec::new(),
            functions: HashMap::new(),
            pc: 0,
        }
    }

    pub fn error(&self, message: &str) {
        eprintln!("{}", message);
        std::process::exit(1);
    }

    fn gen(&mut self, command: Operations) {
        self.program.push(command);
        self.pc = self.pc + 1;
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
                self.gen(Operations::ARG(Value::INT(self.pc + 3)));

                let else_adress = self.pc;

                self.gen(Operations::JMP);
                self.gen(Operations::ARG(Value::INT(0)));

                self.compile(*node.op2.clone().unwrap());

                let after_adress = self.pc;

                self.program[(else_adress + 1) as usize] =
                    Operations::ARG(Value::INT(after_adress));
            }
            Kind::IF_ELSE => {
                self.compile(*node.op1.clone().unwrap());

                self.gen(Operations::JZ);
                self.gen(Operations::ARG(Value::INT(self.pc + 3)));

                let else_jmp_adress = self.pc;

                self.gen(Operations::JMP);
                self.gen(Operations::ARG(Value::INT(0)));

                self.compile(*node.op2.clone().unwrap());

                let complete_adress = self.pc;

                self.gen(Operations::JMP);
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
                self.gen(Operations::ARG(Value::INT(self.pc + 3)));

                let false_jmp_adress = self.pc;

                self.gen(Operations::JMP);
                self.gen(Operations::ARG(Value::INT(0)));

                self.compile(*node.op2.clone().unwrap());

                self.gen(Operations::JMP);
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
                        let function_object = compiler_clone
                            .functions
                            .get(&function_name)
                            .clone()
                            .unwrap();

                        // Implementing arguments

                        let mut args_compiler = Compiler::new();
                        let mut args_bytes = args_compiler
                            .compile(*node.op1.clone().unwrap())
                            .program
                            .into_iter()
                            .collect::<Vec<Operations>>();

                        self.program.append(&mut args_bytes);

                        // Generating variables for arguments

                        for (index, arg) in function_object.arguments.iter().rev().enumerate() {
                            self.gen(Operations::STORE);
                            self.program.push(Operations::ARG(arg.clone()));
                        }

                        let mut function_program = function_object.program.clone();

                        self.program.append(&mut function_program);

                        function_object.arguments.iter().for_each(|arg| {
                            self.gen(Operations::DROP);
                            self.gen(Operations::ARG(arg.clone()))
                        });
                    } else {
                        self.error(
                            format!("Function '{}' is not defined!", &function_name).as_str(),
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
            Kind::RETURN => {
                self.compile(*node.op1.clone().unwrap());
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
        }
    }
}
