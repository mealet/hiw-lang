const VM_CODE: &'static str = r#"
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    NUM(i32),
    STR(String),
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
            let mut arg = Operations::ARG(Value::NUM(0));
            let mut subarg = Operations::ARG(Value::NUM(0));

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
                    // if arg == Operations::ARG( Value::NUM(arg) ) {
                    //     eprintln!("Cannot create variable with number as a name!");
                    // } else if arg.unwrap().len() < 1 {
                    match arg {
                        Operations::ARG(Value::NUM(_)) => {
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
                        Operations::ARG(Value::NUM(_)) => {
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
                        Operations::ARG(Value::NUM(_)) => {
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
                        Value::NUM(integer) => {
                            println!("{}", integer)
                        }
                        Value::STR(string) => {
                            println!("{}", string)
                        }
                    }

                    pc += 1;
                }
                Operations::JMP => {
                    if let Operations::ARG(Value::NUM(jump_code)) = arg {
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
                    if let Operations::ARG(Value::NUM(jump_code)) = arg {
                        if jump_code as usize > self.program.len() {
                            eprintln!("Argument is bigger program length!");
                        } else {
                            let stack_value = self.stack.pop().unwrap();
                            if let Value::NUM(unwrapped_value) = stack_value {
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
                    if let Operations::ARG(Value::NUM(jump_code)) = arg {
                        if jump_code as usize > self.program.len() {
                            eprintln!("Argument is bigger program length!");
                        } else {
                            let stack_value = self.stack.pop().unwrap();
                            if let Value::NUM(unwrapped_value) = stack_value {
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

        if let (Value::NUM(a), Value::NUM(b)) = (_a, _b) {
            self.stack.push(Value::NUM(a + b));
        } else {
            eprintln!("Cannot calculate values which both is not NUM");
        }
    }

    pub fn sub(&mut self) {
        let _a = self.stack.pop().expect("Stack error");
        let _b = self.stack.pop().expect("Stack error");

        if let (Value::NUM(a), Value::NUM(b)) = (_a, _b) {
            self.stack.push(Value::NUM(a / b));
        } else {
            eprintln!("Cannot calculate values which both is not NUM");
        }
    }

    pub fn mult(&mut self) {
        let _a = self.stack.pop().expect("Stack error");
        let _b = self.stack.pop().expect("Stack error");

        if let (Value::NUM(a), Value::NUM(b)) = (_a, _b) {
            self.stack.push(Value::NUM(a * b));
        } else {
            eprintln!("Cannot calculate values which both is not NUM");
        }
    }

    pub fn div(&mut self) {
        let _a = self.stack.pop().expect("Stack error");
        let _b = self.stack.pop().expect("Stack error");

        if let (Value::NUM(a), Value::NUM(b)) = (_a, _b) {
            self.stack.push(Value::NUM(a / b));
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
        let stack_value = self.stack.pop().unwrap_or(Value::NUM(0));
        self.variables.insert(varname, stack_value);
    }
}
"#;

use crate::{lexer::Value, vm::Operations};
use std::{fmt, io::Write};

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::INT(i) => write!(f, "Value::NUM({})", i),
            Value::STR(s) => write!(f, "Value::STR(\"{}\".to_string())", s),
        }
    }
}

impl fmt::Display for Operations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Operations::PUSH => "Operations::PUSH".to_string(),
            Operations::ADD => "Operations::ADD".to_string(),
            Operations::SUB => "Operations::SUB".to_string(),
            Operations::DIV => "Operations::DIV".to_string(),
            Operations::MULT => "Operations::MULT".to_string(),
            Operations::HALT => "Operations::HALT".to_string(),
            Operations::POP => "Operations::POP".to_string(),
            Operations::VAR => "Operations::VAR".to_string(),
            Operations::ARG(value) => format!("Operations::ARG({})", value),
            Operations::FETCH => "Operations::FETCH".to_string(),
            Operations::STORE => "Operations::STORE".to_string(),
            Operations::JMP => "Operations::JMP".to_string(),
            Operations::JZ => "Operations::JZ".to_string(),
            Operations::JNZ => "Operations::JNZ".to_string(),
            Operations::PRINT => "Operations::PRINT".to_string(),
        };
        write!(f, "{}", s)
    }
}

pub struct Container {
    name: String,
    vm: crate::vm::VM,
}

impl Container {
    pub fn new(name: String, vm: crate::vm::VM) -> Self {
        Container { name, vm }
    }
    pub fn compile(self) {
        let filenames = vec![format!("{}.rs", self.name), format!("{}.pdb", self.name)];
        let operations_string_enum: String = self
            .vm
            .program
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let output_code = format!(
            r#"{}
fn main() {{
    let program = vec![{}];
    let mut vm = VM::new(program);

    let _ = vm.run();
}}
"#,
            VM_CODE, operations_string_enum
        );

        let mut output_file = std::fs::File::create(&filenames[0]).unwrap();
        let _ = output_file.write_all(output_code.as_bytes());

        let compiler = std::process::Command::new("rustc")
            .arg(&filenames[0])
            .output()
            .expect("Cannot compile VM");

        for fname in filenames {
            let _ = std::fs::remove_file(fname);
        }

        println!("COMPILE MODE | {} successfuly compiled!", &self.name);
    }
}
