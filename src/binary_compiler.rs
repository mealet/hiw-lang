const VM_CODE: &'static str = r#"
use std::collections::HashMap;
use colored::Colorize;

type PROGRAM = Vec<Operations>;



#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    INT(i32),
    STR(String),
    BOOL(bool),
    ARRAY(Vec<Value>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct VM {
    pub stack: Vec<Value>,
    pub program: PROGRAM,
    pub variables: HashMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operations {
    PUSH,
    //
    ARR,
    SLICE,
    //
    ADD,
    SUB,
    DIV,
    MULT,
    //
    VAR,
    ARG(Value),
    FETCH,
    STORE,
    //
    PRINT,
    INPUT,
    //
    LT,
    BT,
    EQ,
    //
    JMP,
    JZ,
    JNZ,
    //
    DROP,
    POP,
    HALT,
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
        m.insert("HALT", Operations::HALT);
        m
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub name: Value,
    pub arguments: Vec<Value>,
    pub program: PROGRAM,
}

impl VM {
    pub fn new(program: PROGRAM) -> Self {
        VM {
            stack: Vec::new(),
            program: program,
            variables: HashMap::new(),
        }
    }

    fn error(&self, message: &str) {
        eprintln!("{} {}", "[RuntimeError]".red(), message);
        std::process::exit(1);
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
                    let _b = self.stack.pop().expect("Stack error");
                    let _a = self.stack.pop().expect("Stack error");

                    match (_a, _b) {
                        // Both same types
                        (Value::INT(a), Value::INT(b)) => self.stack.push(Value::INT(a + b)),
                        (Value::STR(a), Value::STR(b)) => {
                            self.stack.push(Value::STR(format!("{}{}", a, b)));
                        }
                        (Value::ARRAY(a), Value::ARRAY(b)) => {
                            let mut _temp_a = a.clone();
                            let mut _temp_b = b.clone();

                            let temp_array = _temp_a.append(&mut _temp_b);
                        }
                        (Value::BOOL(a), Value::BOOL(b)) => {
                            let boolean_value = match (a, b) {
                                (true, true) => true,
                                (false, false) => false,
                                (true, false) => true,
                                (false, true) => false,
                            };

                            self.stack.push(Value::BOOL(boolean_value));
                        }

                        // INT and STR
                        (Value::INT(a), Value::STR(b)) => {
                            self.stack.push(Value::STR(format!("{}{}", a, b)));
                        }
                        (Value::STR(a), Value::INT(b)) => {
                            self.stack.push(Value::STR(format!("{}{}", a, b)));
                        }

                        // BOOL and STR
                        (Value::BOOL(a), Value::STR(b)) => {
                            self.stack.push(Value::STR(format!("{}{}", a, b)));
                        }
                        (Value::STR(a), Value::BOOL(b)) => {
                            self.stack.push(Value::STR(format!("{}{}", a, b)));
                        }

                        // ARRAY and STR
                        (Value::ARRAY(a), Value::STR(b)) => {
                            let mut values_array: Vec<String> = Vec::new();

                            for item in a {
                                let printable_value = match item {
                                    Value::INT(i) => &i.to_string(),
                                    Value::STR(s) => &format!("\"{}\"", s),
                                    Value::BOOL(b) => &b.to_string(),
                                    Value::ARRAY(_) => &("ARRAY[]".to_string()),
                                };

                                values_array.push(printable_value.clone());
                            }

                            let _f = format!("[{}]{}", values_array.join(","), b);
                        }
                        (Value::STR(a), Value::ARRAY(b)) => {
                            let mut values_array: Vec<String> = Vec::new();

                            for item in b {
                                let printable_value = match item {
                                    Value::INT(i) => &i.to_string(),
                                    Value::STR(s) => &format!("\"{}\"", s),
                                    Value::BOOL(b) => &b.to_string(),
                                    Value::ARRAY(_) => &("ARRAY[]".to_string()),
                                };

                                values_array.push(printable_value.clone());
                            }

                            let _f = format!("[{}]{}", values_array.join(","), a);
                        }

                        // Other values we cannot implement
                        _ => self.error("Cannot add not implemented values!"),
                    }

                    pc += 1
                }
                Operations::SUB => {
                    let _b = self.stack.pop().expect("Stack error");
                    let _a = self.stack.pop().expect("Stack error");

                    match (_a, _b) {
                        (Value::INT(a), Value::INT(b)) => {
                            self.stack.push(Value::INT(a - b));
                        }
                        _ => self.error("Cannot substract types which doesn't implemented!"),
                    };

                    pc += 1
                }
                Operations::MULT => {
                    let _b = self.stack.pop().expect("Stack error");
                    let _a = self.stack.pop().expect("Stack error");

                    match (_a, _b) {
                        // Same type
                        (Value::INT(a), Value::INT(b)) => {
                            self.stack.push(Value::INT(a * b));
                        }

                        // INT and STR
                        (Value::INT(a), Value::STR(b)) => {
                            self.stack.push(Value::STR(b.repeat(a as usize)));
                        }
                        (Value::STR(a), Value::INT(b)) => {
                            self.stack.push(Value::STR(a.repeat(b as usize)));
                        }

                        // INT and ARRAY
                        (Value::INT(a), Value::ARRAY(b)) => {
                            let mut _temp_b = b.clone();
                            let mut _arr = Vec::new();

                            for _ in 0..a {
                                _arr.append(&mut _temp_b);
                            }

                            self.stack.push(Value::ARRAY(_arr));
                        }
                        (Value::ARRAY(a), Value::INT(b)) => {
                            let mut _temp_a = a.clone();
                            let mut _arr = Vec::new();

                            for _ in 0..b {
                                _arr.append(&mut _temp_a);
                            }

                            self.stack.push(Value::ARRAY(_arr));
                        }

                        // Others
                        _ => self.error("Cannot multiply types which doesn't implemented!"),
                    }

                    pc += 1
                }
                Operations::DIV => {
                    let _b = self.stack.pop().expect("Stack error");
                    let _a = self.stack.pop().expect("Stack error");

                    match (_a, _b) {
                        // Same type
                        (Value::INT(a), Value::INT(b)) => {
                            self.stack.push(Value::INT(a / b));
                        }

                        // INT and STR
                        (Value::STR(a), Value::INT(b)) => {
                            if a.len() < 1 {
                                self.error("Cannot divide string which length is less 2");
                            }

                            let final_string_length = a.len() / 2;
                            let _chars = a
                                .clone()
                                .chars()
                                .into_iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>();

                            // Doing a crutch because rust cannot let me just use slices (cuz size is unknown at
                            // compilation time 🤬)

                            let mut _str = String::new();

                            for index in 0..final_string_length {
                                _str.push_str(_chars[index].as_str());
                            }

                            // FINALLY PUSHING IT TO STACK

                            self.stack.push(Value::STR(_str));
                        }

                        // Others
                        _ => self.error("Cannot divide types which doesn't implemented!"),
                    }

                    pc += 1
                }
                Operations::POP => {
                    self.stack.pop();
                    pc += 1
                }
                Operations::DROP => {
                    match arg {
                        Operations::ARG(Value::STR(val)) => {
                            if self.variables.contains_key(&val) {
                                self.variables.remove(&val);
                            } else {
                                self.error(
                                    format!("Value '{}' being dropped does not exists!", &val)
                                        .as_str(),
                                );
                            }
                        }
                        _ => {
                            self.error("Dropping value isn't ID!");
                        }
                    };
                    pc += 2;
                }
                Operations::PUSH => {
                    match arg {
                        Operations::ARG(a) => {
                            self.stack.push(a);
                        }
                        _ => {
                            eprintln!("Error occured while managin' data in stack");
                        }
                    }

                    pc += 2
                }
                Operations::VAR => {
                    // if arg == Operations::ARG( Value::INT(arg) ) {
                    //     eprintln!("Cannot create variable with number as a name!");
                    // } else if arg.unwrap().len() < 1 {
                    match arg {
                        Operations::ARG(Value::INT(_)) => {
                            self.error("Cannot create variable with NUMBER as a name!");
                        }
                        Operations::ARG(Value::STR(varname)) => {
                            if varname.len() < 1 {
                                self.error("Unexpected variable name!");
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
                            self.error("Undefined variable name!");
                        }
                    }

                    pc += 3;
                }
                Operations::FETCH => {
                    match arg {
                        Operations::ARG(Value::INT(_)) => {
                            self.error("Cannot get data from variable NUMBER as a name!");
                        }
                        Operations::ARG(Value::STR(varname)) => {
                            if varname.len() < 1 {
                                self.error("Unexpected variable name!");
                            } else {
                                if !(self.variables.contains_key(varname.as_str())) {
                                    self.error(
                                        format!("Variable '{}' is not defined!", varname).as_str(),
                                    );
                                }

                                let variable_value =
                                    self.variables.clone().get(&varname).unwrap().clone();
                                let _ = self.stack.push(variable_value);
                            }
                        }
                        _ => {
                            self.error("Variable name must be alphanumeric!");
                        }
                    }

                    pc += 2;
                }
                Operations::STORE => {
                    match arg {
                        Operations::ARG(Value::INT(_)) => {
                            self.error("Cannot store data to variable which name is number!");
                        }
                        Operations::ARG(Value::STR(varname)) => {
                            if varname.len() < 1 {
                                self.error("Unexpected variable name!");
                            } else {
                                let stack_value = self.stack.pop().unwrap_or(Value::INT(0));
                                self.variables.insert(varname, stack_value);
                            }
                        }
                        _ => {
                            self.error("Unexpected operation!");
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
                        Value::ARRAY(array) => {
                            print!("\n[");

                            for (index, item) in array.iter().enumerate() {
                                let printable_value = match item {
                                    Value::INT(i) => &i.to_string(),
                                    Value::STR(s) => &format!("\"{}\"", s),
                                    Value::BOOL(b) => &b.to_string(),
                                    Value::ARRAY(_) => &("ERR".to_string()),
                                };

                                print!("{}", printable_value);

                                if index != array.len() - 1 {
                                    print!(", ");
                                }
                            }

                            print!("]");
                        }
                    }

                    pc += 1;
                }
                Operations::INPUT => {
                    let mut input_string = String::new();
                    let _ = std::io::stdin().read_line(&mut input_string);

                    self.stack.push(Value::STR(input_string.trim().to_string()));

                    pc += 1;
                }
                Operations::JMP => {
                    if let Operations::ARG(Value::INT(jump_code)) = arg {
                        if jump_code as usize > self.program.len() {
                            self.error("Jump Code is bigger than byte code!");
                            pc += 2;
                        } else {
                            pc = jump_code as usize;
                        }
                    } else {
                        self.error("Jump Code isn't number!");
                        pc += 2;
                    }
                }
                Operations::JZ => {
                    if let Operations::ARG(Value::INT(jump_code)) = arg {
                        if jump_code as usize > self.program.len() {
                            self.error("Jump Code is bigger than byte code!");
                        } else {
                            let stack_value = self.stack.pop().unwrap_or_else(|| {
                                println!("{:?}", self.stack);
                                self.error("Stack error with boolean operation!");
                                Value::BOOL(false)
                            });
                            if let Value::BOOL(unwrapped_value) = stack_value {
                                if unwrapped_value == true {
                                    pc = jump_code as usize;
                                } else {
                                    pc += 2;
                                }
                            } else {
                                self.error("Comparsion result isn't boolean!");
                            }
                        }
                    } else {
                        self.error("Jump Code isn't number!");
                    }
                }
                Operations::JNZ => {
                    if let Operations::ARG(Value::INT(jump_code)) = arg {
                        if jump_code as usize > self.program.len() {
                            self.error("Jump Code is bigger than byte code!");
                        } else {
                            let stack_value = self.stack.pop().unwrap();
                            if let Value::BOOL(unwrapped_value) = stack_value {
                                if unwrapped_value != true {
                                    pc = jump_code as usize
                                } else {
                                    pc += 2
                                }
                            } else {
                                self.error("Comparsion result isn't boolean!");
                            }
                        }
                    } else {
                        eprintln!("Argument must be number!");
                    }
                }
                Operations::LT => {
                    let right_stack = self.stack.pop().unwrap();
                    let left_stack = self.stack.pop().unwrap();

                    match (left_stack.clone(), right_stack.clone()) {
                        (Value::INT(left), Value::INT(right)) => {
                            if left < right {
                                self.stack.push(Value::BOOL(true));
                            } else {
                                self.stack.push(Value::BOOL(false));
                            }
                        }
                        (Value::STR(left), Value::STR(right)) => {
                            if left.len() < right.len() {
                                self.stack.push(Value::BOOL(true));
                            } else {
                                self.stack.push(Value::BOOL(false));
                            }
                        }
                        (Value::ARRAY(left), Value::ARRAY(right)) => {
                            if left.len() < right.len() {
                                self.stack.push(Value::BOOL(true));
                            } else {
                                self.stack.push(Value::BOOL(false));
                            }
                        }
                        _ => {
                            self.error(
                                format!(
                                    "Cannot compare {:?} and {:?}. Unexpected types.",
                                    left_stack, right_stack
                                )
                                .as_str(),
                            );
                        }
                    }

                    pc += 1
                }
                Operations::BT => {
                    let right_stack = self.stack.pop().unwrap();
                    let left_stack = self.stack.pop().unwrap();

                    match (left_stack.clone(), right_stack.clone()) {
                        (Value::INT(left), Value::INT(right)) => {
                            if left > right {
                                self.stack.push(Value::BOOL(true));
                            } else {
                                self.stack.push(Value::BOOL(false));
                            }
                        }
                        (Value::STR(left), Value::STR(right)) => {
                            if left.len() > right.len() {
                                self.stack.push(Value::BOOL(true));
                            } else {
                                self.stack.push(Value::BOOL(false));
                            }
                        }
                        (Value::ARRAY(left), Value::ARRAY(right)) => {
                            if left.len() > right.len() {
                                self.stack.push(Value::BOOL(true));
                            } else {
                                self.stack.push(Value::BOOL(false));
                            }
                        }
                        _ => {
                            self.error(
                                format!(
                                    "Cannot compare {:?} and {:?}. Unexpected types.",
                                    left_stack, right_stack
                                )
                                .as_str(),
                            );
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
                Operations::ARR => {
                    let mut array_result = Vec::new();

                    for i in self.stack.clone().iter() {
                        array_result.push(i.clone());
                        self.stack.pop();
                    }

                    for _ in 0..self.stack.len() {
                        array_result.push(self.stack.pop().unwrap());
                    }

                    self.stack.push(Value::ARRAY(array_result));

                    pc += 1;
                }
                Operations::SLICE => {
                    // Slice from value at the top of stack

                    let stack_argument = self.stack.pop().unwrap();
                    let slicable_object = self.stack.pop().unwrap();

                    arg = Operations::ARG(stack_argument);

                    match arg {
                        Operations::ARG(Value::INT(slice_index)) => match slicable_object {
                            Value::STR(slicable_string) => {
                                let string_vector = slicable_string.chars().collect::<Vec<_>>();

                                self.stack.push(Value::STR(
                                    string_vector[slice_index as usize].to_string(),
                                ));
                            }
                            Value::ARRAY(slicable_array) => {
                                self.stack
                                    .push(slicable_array[slice_index as usize].clone());
                            }
                            _ => self.error("Cannot get slice from type exclude STR or ARRAY"),
                        },
                        _ => {
                            self.error("Cannot get slice of non-integer index!");
                        }
                    };

                    pc += 1;
                }
                Operations::HALT => break,
                _ => {
                    eprintln!("Undefined operation: {:?}!", &self.program[pc]);
                    pc += 1
                }
            }
        }

        return Ok(());
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
            Value::ARRAY(a) => write!(f, "Value::ARRAY({:?})", a),
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
            Operations::ARR => "Operations::ARR".to_string(),
            Operations::SLICE => "Operations::SLICE".to_string(),
            Operations::DROP => "Operations::DROP".to_string(),
            Operations::INPUT => "Operations::INPUT".to_string(),
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

        println!("{:?}", String::from_utf8_lossy(&compiler.stderr));

        for fname in filenames {
            let _ = std::fs::remove_file(fname);
        }

        println!("COMPILE MODE | {} successfuly compiled!", &self.name);
    }
}
