// hiw-lang compiler
// https://github.com/mealet/hiw-lang
// ----------------------------------------
// Copyright ©️ 2024, mealet.
// Project licensed under the BSD-3 License
// that can be found in LICENSE file.
// ----------------------------------------

#[allow(unused, dead_code)]
#[macro_use]
extern crate lazy_static;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

use colored::Colorize;

mod binary_compiler;
mod compiler;
mod filereader;
mod greeting;
mod lexer;
mod parser;
mod vm;

fn main() {
    // Option Variables

    let mut compile_mode = false;

    // Greeting user

    greeting::greeting();

    // Getting args
    let args: Vec<String> = std::env::args().collect();

    if args.clone().len() < 2 {
        eprintln!(
            "| Usage for compiling and running: {}\n|-- Example: {}\n|\n| Usage for compiling to binary file: {}\n|-- Example: {}",
            format!("{} [file]", APP_NAME).red(), format!("{} example.hiw", APP_NAME).red(), format!("{} [file] [output]", APP_NAME).red(), format!("{} example.hiw output", APP_NAME).red()
        );
        std::process::exit(1);
    } else if args.clone().len() > 2 {
        compile_mode = true;
    }

    let filepath = std::path::Path::new(&args[1]);
    let filename = filepath.file_name().unwrap();

    // Creating Lexer Analyzer

    let input = filereader::get_code(filepath.to_str().unwrap().to_string());
    let lexer = lexer::Lexer::new(input, filename.to_str().unwrap().to_string());

    // Checking lexical analyzer errors

    let mut lexer_clone = lexer.clone();
    while lexer_clone.token != Some(lexer::Token::EOF) {
        lexer_clone.next_token();
    }

    if lexer_clone.errors.len() > 0 {
        for err in lexer_clone.errors {
            eprintln!("{}", err);
        }
        std::process::exit(1);
    }

    // Parsing Lexer Tokens

    let mut parser = parser::Parser::new(lexer);
    let abstract_syntax_tree = parser.parse();

    // Checking parser errors

    if parser.errors.len() > 0 {
        for err in parser.errors {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }

    // Compiling Tree to byte code

    let mut compiler = compiler::Compiler::new();
    let byte_code = compiler.compile_all(abstract_syntax_tree);

    // Creating VM

    let mut vm = vm::VM::new(byte_code.program);

    // Checking compile mode

    if compile_mode {
        // Compiling VM with commands

        let output_filename = args[2].clone();

        let compile_container = binary_compiler::Container::new(output_filename, vm);
        let _ = compile_container.compile();
    } else {
        // Running VM

        let _ = vm.run();
    }
}

// TODO: Add functions:
//              <ARRAY>.push(arg)
//              <ARRAY>.join(<STR>)
//
// TODO: Change binary compiler code (must read vm code from compiler folder)
//
// FIXME: Array in Array concatenating wrong
