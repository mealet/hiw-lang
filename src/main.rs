// HIW Compiler v1.0.0
// HIW - Holy shit It Works
// Simple compiler created for training in Rust Language
// =====================================================
// https://github.com/mealet

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

    // Creating Lexer Analyzer

    let input = filereader::get_code(args[1].clone());
    let lexer = lexer::Lexer::new(input);

    // Parsing Lexer Tokens

    let mut parser = parser::Parser::new(lexer);
    let abstract_syntax_tree = parser.parse();

    println!("{:?}", abstract_syntax_tree);

    // // Compiling Tree to byte code
    //
    // let mut compiler = compiler::Compiler::new();
    // let byte_code = compiler.compile(abstract_syntax_tree);
    //
    // // Creating VM
    //
    // let mut vm = vm::VM::new(byte_code.program);
    //
    // // Checking compile mode
    //
    // if compile_mode {
    //     // Compiling VM with commands
    //
    //     let output_filename = args[2].clone();
    //
    //     let compile_container = binary_compiler::Container::new(output_filename, vm);
    //     let _ = compile_container.compile();
    // } else {
    //     // Running VM
    //
    //     let _ = vm.run();
    // }
}

// TODO: Create built-in functions with op!() macro and VM Bytes
// FIXME: While cycle is still going infinite loop
