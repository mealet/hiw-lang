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
            Kind::VAR => {
                self.gen(Operations::FETCH);
                self.gen(Operations::ARG(node.value.unwrap()));
            }
            Kind::CONST => {
                self.gen(Operations::PUSH);
                self.gen(Operations::ARG(node.value.unwrap()));
            }
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
            Kind::SET => {
                self.compile(*node.op2.clone().unwrap());
                self.gen(Operations::STORE);
                self.gen(Operations::ARG(node.op1.clone().unwrap().value.unwrap()));
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

