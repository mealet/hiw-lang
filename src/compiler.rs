// Compiler - magic wand which converts AST to virtual machine code.

use crate::{
    lexer::Value,
    parser::{Kind, Node},
    vm::Operations,
};

pub struct Compiler {
    program: Vec<Operations>,
    pc: i32,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            program: Vec::new(),
            pc: 0,
        }
    }

    fn gen(&mut self, command: Operations) {
        self.program.push(command);
        self.pc = self.pc + 1;
    }

    pub fn compile(&mut self, node: Node) -> Vec<Operations> {
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

            Kind::BRACK_ENUM => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());
            }
            Kind::SLICE => {
                self.compile(*node.op1.clone().unwrap());
                self.compile(*node.op2.clone().unwrap());

                self.gen(Operations::SLICE);
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

        self.program.clone()
    }
}
