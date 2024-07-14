// hiw-lang compiler
// https://github.com/mealet/hiw-lang
// ----------------------------------------
// Copyright ©️ 2024, mealet.
// Project licensed under the BSD-3 License
// that can be found in LICENSE file.
// ----------------------------------------

// Binary Compiler - module, which created to wrap virtual-machine and compiled byte-code to
// executable file

use crate::{vm::Operations, vm::Value};
use colored::Colorize;
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
            Operations::TYPE => "Operations::TYPE".to_string(),
            Operations::LEN => "Operations::LEN".to_string(),
            Operations::TO_INT => "Operations::TO_INT".to_string(),
            Operations::TO_STR => "Operations::TO_STR".to_string(),
            Operations::CLEAN => "Operations::CLEAN".to_string(),
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

        let vm_code = crate::filereader::get_vm_code();

        let output_code = format!(
            r#"{}
fn main() {{
    let program = vec![{}];
    let mut vm = VM::new(program);

    let _ = vm.run();
}}
"#,
            vm_code, operations_string_enum
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

        match compiler.status.code() {
            Some(0) => {
                println!(
                    "{} '{}' successfuly compiled!",
                    "[BinaryCompiler]:".cyan(),
                    &self.name
                );
            }
            _ => {
                eprintln!(
                    "{} An error occured while compiling '{}'",
                    "[BinaryCompiler]:".red(),
                    &self.name
                )
            }
        }
    }
}
