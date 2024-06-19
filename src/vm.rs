// VM (virtual machine) - low level "computer" that gives me tool for converting AST to byte code
// and running it on this VM

use crate::lexer::Value;
use std::collections::HashMap;

type PROGRAM = Vec<Operations>;

#[derive(Debug, PartialEq, Eq)]
pub struct VM {
    pub stack: Vec<Value>,
    pub program: PROGRAM,
    pub variables: HashMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operations {
    PUSH,
    ADD,
    SUB,
    DIV,
    MULT,
    HALT,
    POP,
    VAR,
    ARG(Value),
    FETCH,
    STORE,
    PRINT,
    JMP,
    JZ,
    JNZ,
}

impl VM {
    pub fn new(prog: PROGRAM) -> Self {
        VM {
            stack: Vec::new(),
            program: prog,
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), &str> {
        let mut pc: usize = 0;

        loop {
            let mut arg = Operations::ARG(Value::INT(0));
            let mut subarg = Operations::ARG(Value::INT(0));

            if pc < self.program.len() - 1 {
                arg = self.program[pc + 1].clone();

                if pc < self.program.len() - 2 {
                    subarg = self.program[pc + 2].clone();
                }
            }

            match self.program[pc] {
                Operations::ADD => {
                    self.add();
                    pc += 1
                }
                Operations::SUB => {
                    self.sub();
                    pc += 1
                }
                Operations::MULT => {
                    self.mult();
                    pc += 1
                }
                Operations::DIV => {
                    self.div();
                    pc += 1
                }
                Operations::POP => {
                    self.pop();
                    pc += 1
                }
                Operations::PUSH => {
                    self.push(arg);
                    pc += 2
                }
                Operations::HALT => break,
                Operations::VAR => {
                    // if arg == Operations::ARG( Value::INT(arg) ) {
                    //     eprintln!("Cannot create variable with number as a name!");
                    // } else if arg.unwrap().len() < 1 {
                    match arg {
                        Operations::ARG(Value::INT(_)) => {
                            eprintln!("Cannot create variable with number as a name!");
                        }
                        Operations::ARG(Value::STR(varname)) => {
                            if varname.len() < 1 {
                                eprintln!("Unexpected variable name!");
                            } else {
                                match subarg {
                                    Operations::ARG(t) => {
                                        self.variables.insert(varname, t);
                                    }
                                    _ => {
                                        eprintln!(
                                            "Cannot create variable {}! No values found!",
                                            varname
                                        );
                                    }
                                }
                            }
                        }
                        _ => {
                            eprintln!("Undefined variable name! Skipping operations...");
                        }
                    }

                    pc += 3;
                }
                Operations::FETCH => {
                    match arg {
                        Operations::ARG(Value::INT(_)) => {
                            eprintln!("Cannot fetch value from variable with number as a name!");
                        }
                        Operations::ARG(Value::STR(varname)) => {
                            if varname.len() < 1 {
                                eprintln!("Unexpected variable name!");
                            } else {
                                self.fetch(varname);
                            }
                        }
                        _ => {
                            eprintln!("Undefined operation!")
                        }
                    }

                    pc += 2;
                }
                Operations::STORE => {
                    match arg {
                        Operations::ARG(Value::INT(_)) => {
                            eprintln!("Cannot value to variable with number as a name!");
                        }
                        Operations::ARG(Value::STR(varname)) => {
                            if varname.len() < 1 {
                                eprintln!("Unexpected variable name!");
                            } else {
                                self.store(varname);
                            }
                        }
                        _ => {
                            eprintln!("Undefined operation!")
                        }
                    }

                    pc += 2
                }
                Operations::PRINT => {
                    let print_value = self.stack.pop().unwrap();
                    match print_value {
                        Value::INT(integer) => {
                            println!("{}", integer)
                        }
                        Value::STR(string) => {
                            println!("{}", string)
                        }
                    }

                    pc += 1;
                }
                Operations::JMP => {
                    if let Operations::ARG(Value::INT(jump_code)) = arg {
                        if jump_code as usize > self.program.len() {
                            eprintln!("Argument is bigger program length!");
                        } else {
                            pc = jump_code as usize;
                        }
                    } else {
                        eprintln!("Argument must be number!");
                    }
                }
                Operations::JZ => {
                    if let Operations::ARG(Value::INT(jump_code)) = arg {
                        if jump_code as usize > self.program.len() {
                            eprintln!("Argument is bigger program length!");
                        } else {
                            let stack_value = self.stack.pop().unwrap();
                            if let Value::INT(unwrapped_value) = stack_value {
                                if unwrapped_value == 0 {
                                    pc = jump_code as usize
                                } else {
                                    pc += 1
                                }
                            } else {
                                eprintln!("Stack value at the top is not NUMBER!");
                            }
                        }
                    } else {
                        eprintln!("Argument must be number!");
                    }
                }
                Operations::JZ => {
                    if let Operations::ARG(Value::INT(jump_code)) = arg {
                        if jump_code as usize > self.program.len() {
                            eprintln!("Argument is bigger program length!");
                        } else {
                            let stack_value = self.stack.pop().unwrap();
                            if let Value::INT(unwrapped_value) = stack_value {
                                if unwrapped_value == 0 {
                                    pc = jump_code as usize
                                } else {
                                    pc += 1
                                }
                            } else {
                                eprintln!("Stack value at the top is not NUMBER!");
                            }
                        }
                    } else {
                        eprintln!("Argument must be number!");
                    }
                }
                _ => {
                    eprintln!("Undefined operation with number {}! Skipping...", pc);
                    pc += 1
                }
            }
        }

        return Ok(());
    }

    // Commands

    pub fn add(&mut self) {
        let _a = self.stack.pop().expect("Stack error");
        let _b = self.stack.pop().expect("Stack error");

        if let (Value::INT(a), Value::INT(b)) = (_a, _b) {
            self.stack.push(Value::INT(a + b));
        } else {
            eprintln!("Cannot calculate values which both is not NUM");
        }
    }

    pub fn sub(&mut self) {
        let _a = self.stack.pop().expect("Stack error");
        let _b = self.stack.pop().expect("Stack error");

        if let (Value::INT(a), Value::INT(b)) = (_a, _b) {
            self.stack.push(Value::INT(a / b));
        } else {
            eprintln!("Cannot calculate values which both is not NUM");
        }
    }

    pub fn mult(&mut self) {
        let _a = self.stack.pop().expect("Stack error");
        let _b = self.stack.pop().expect("Stack error");

        if let (Value::INT(a), Value::INT(b)) = (_a, _b) {
            self.stack.push(Value::INT(a * b));
        } else {
            eprintln!("Cannot calculate values which both is not NUM");
        }
    }

    pub fn div(&mut self) {
        let _a = self.stack.pop().expect("Stack error");
        let _b = self.stack.pop().expect("Stack error");

        if let (Value::INT(a), Value::INT(b)) = (_a, _b) {
            self.stack.push(Value::INT(a / b));
        } else {
            eprintln!("Cannot calculate values which both is not NUM");
        }
    }

    pub fn push(&mut self, arg: Operations) {
        match arg {
            Operations::ARG(a) => {
                self.stack.push(a);
            }
            _ => {
                eprintln!("Error while pushing argument to stack")
            }
        }
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn fetch(&mut self, varname: String) {
        if !(self.variables.contains_key(varname.as_str())) {
            eprintln!("Variable '{}' does not exists!", varname);
            return;
        }

        let variable_value = self.variables[varname.as_str()].clone();
        let _ = self.variables.remove(varname.as_str());

        let _ = self.stack.push(variable_value);
    }

    pub fn store(&mut self, varname: String) {
        let stack_value = self.stack.pop().unwrap_or(Value::INT(0));
        self.variables.insert(varname, stack_value);
    }
}
