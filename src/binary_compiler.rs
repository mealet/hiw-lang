const VM_CODE: &'static str = r#"
use std::collections::HashMap;

type PROGRAM = Vec<Operations>;

#[derive(Debug, PartialEq, Eq)]
pub struct VM {
    pub stack: Vec<Value>,
    pub program: PROGRAM,
    pub variables: HashMap<String, Value>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    INT(i32),
    STR(String),
    BOOL(bool),
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
    LT,
    BT,
    EQ,
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
                        Value::STR(string) => match string.as_str() {
                            "::stack" => println!("{:?}", self.stack.clone()),
                            "::var" => println!("{:?}", self.variables.clone()),
                            _ => println!("{}", string),
                        },
                        Value::BOOL(boo) => {
                            if boo {
                                println!("true");
                            } else {
                                println!("false");
                            }
                        }
                    }

                    pc += 1;
                }
                Operations::JMP => {
                    if let Operations::ARG(Value::INT(jump_code)) = arg {
                        if jump_code as usize > self.program.len() {
                            eprintln!("Argument is bigger program length!");
                            pc += 2;
                        } else {
                            pc = jump_code as usize;
                        }
                    } else {
                        eprintln!("Argument must be number!");
                        pc += 2;
                    }
                }
                Operations::JZ => {
                    if let Operations::ARG(Value::INT(jump_code)) = arg {
                        if jump_code as usize > self.program.len() {
                            eprintln!("Argument is bigger program length!");
                        } else {
                            let stack_value = self.stack.pop().unwrap();
                            if let Value::BOOL(unwrapped_value) = stack_value {
                                if unwrapped_value == true {
                                    pc = jump_code as usize
                                } else {
                                    pc += 2
                                }
                            } else {
                                eprintln!("Stack value at the top is not BOOL!");
                            }
                        }
                    } else {
                        eprintln!("Argument must be number!");
                    }
                }
                Operations::JNZ => {
                    if let Operations::ARG(Value::INT(jump_code)) = arg {
                        if jump_code as usize > self.program.len() {
                            eprintln!("Argument is bigger program length!");
                        } else {
                            let stack_value = self.stack.pop().unwrap();
                            if let Value::BOOL(unwrapped_value) = stack_value {
                                if unwrapped_value != true {
                                    pc = jump_code as usize
                                } else {
                                    pc += 2
                                }
                            } else {
                                eprintln!("Stack value at the top is not BOOL!");
                            }
                        }
                    } else {
                        eprintln!("Argument must be number!");
                    }
                }
                Operations::LT => {
                    let right_stack = self.stack.pop().unwrap();
                    let left_stack = self.stack.pop().unwrap();

                    if let (Value::INT(left), Value::INT(right)) = (left_stack, right_stack) {
                        if left < right {
                            self.stack.push(Value::BOOL(true));
                        } else {
                            self.stack.push(Value::BOOL(false));
                        }
                    }

                    pc += 1
                }
                Operations::BT => {
                    let right_stack = self.stack.pop().unwrap();
                    let left_stack = self.stack.pop().unwrap();

                    if let (Value::INT(left), Value::INT(right)) = (left_stack, right_stack) {
                        if left > right {
                            self.stack.push(Value::BOOL(true));
                        } else {
                            self.stack.push(Value::BOOL(false));
                        }
                    }

                    pc += 1
                }
                Operations::EQ => {
                    let right_stack = self.stack.pop().unwrap();
                    let left_stack = self.stack.pop().unwrap();

                    if left_stack == right_stack {
                        self.stack.push(Value::BOOL(true));
                    } else {
                        self.stack.push(Value::BOOL(false));
                    }

                    pc += 1;
                }
                _ => {
                    eprintln!(
                        "Undefined operation with number {:?}! Skipping...",
                        &self.program[pc]
                    );
                    pc += 1
                }
            }
        }

        return Ok(());
    }

    // Commands

    pub fn add(&mut self) {
        let _b = self.stack.pop().expect("Stack error");
        let _a = self.stack.pop().expect("Stack error");

        if let (Value::INT(a), Value::INT(b)) = (_a, _b) {
            self.stack.push(Value::INT(a + b));
        } else {
            eprintln!("Cannot calculate values which both is not NUM");
        }
    }

    pub fn sub(&mut self) {
        let _b = self.stack.pop().expect("Stack error");
        let _a = self.stack.pop().expect("Stack error");

        if let (Value::INT(a), Value::INT(b)) = (_a, _b) {
            self.stack.push(Value::INT(a / b));
        } else {
            eprintln!("Cannot calculate values which both is not NUM");
        }
    }

    pub fn mult(&mut self) {
        let _b = self.stack.pop().expect("Stack error");
        let _a = self.stack.pop().expect("Stack error");

        if let (Value::INT(a), Value::INT(b)) = (_a, _b) {
            self.stack.push(Value::INT(a * b));
        } else {
            eprintln!("Cannot calculate values which both is not NUM");
        }
    }

    pub fn div(&mut self) {
        let _b = self.stack.pop().expect("Stack error");
        let _a = self.stack.pop().expect("Stack error");

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
            eprintln!("Variable '{}' is not defined!", varname);
            return;
        }

        let variable_value = self.variables[varname.as_str()].clone();
        let _ = self.stack.push(variable_value);
    }

    pub fn store(&mut self, varname: String) {
        let stack_value = self.stack.pop().unwrap_or(Value::INT(0));
        self.variables.insert(varname, stack_value);
    }
}
"#;

use crate::{lexer::Value, vm::Operations};
use std::{fmt, io::Write};

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::INT(i) => write!(f, "Value::INT({})", i),
            Value::STR(s) => write!(f, "Value::STR(\"{}\".to_string())", s),
            Value::BOOL(b) => write!(f, "Value::BOOL({})", b),
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
            Operations::LT => "Operations::LT".to_string(),
            Operations::BT => "Operations::BT".to_string(),
            Operations::EQ => "Operations::EQ".to_string(),
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
