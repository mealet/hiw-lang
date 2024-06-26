// HIW Compiler v1.0.0
// HIW - Holy shit It Works
// Simple compiler created for training in Rust Language
// =====================================================
// https://github.com/mealet

#[allow(unused, dead_code)]
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
        eprintln!("Not enough arguments!");
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

    // Compiling Tree to byte code

    let mut compiler = compiler::Compiler::new();
    let byte_code = compiler.compile(abstract_syntax_tree);

    // Creating VM

    let mut vm = vm::VM::new(byte_code);

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

// TODO: Add some built-in functions (sin, cos, tg, ctg, abs, int, string, bool, input, type)
// TODO: Add creating custom functions

// FIXME: Cannot add string or any other type to array (getting looped)
// TODO: Add getting values from arrays

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    // Lexer Tests;
    #[test]
    fn math_tokens_test() {
        let input = String::from("print(\"Hello world!\"); print(\"Hello World!\");");

        let mut lexer = lexer::Lexer::new(input);

        let mut tokens = Vec::new();

        while lexer.token != Some(lexer::Token::EOF) {
            lexer.next_token();
            if let Some(tok) = lexer.token {
                let _ = tokens.push(tok);
            }
        }

        println!("{:?}", tokens);
    }

    #[test]
    fn variable_tokens_test() {
        let input = String::from("a = 5; b = 3;");
        let mut lexer = lexer::Lexer::new(input);

        let mut tokens = Vec::new();
        let mut values = Vec::new();

        while lexer.token != Some(lexer::Token::EOF) {
            lexer.next_token();
            if let Some(tok) = lexer.token {
                let _ = tokens.push(tok);
                let _ = values.push(lexer.value.clone());
            }
        }

        println!("{:?}", tokens);
        println!("{:?}", values);
    }

    #[test]
    fn print_tokens_test() {
        let input = String::from("print(\"fd!\")");
        let mut lexer = lexer::Lexer::new(input.clone());

        let mut tokens = Vec::new();
        let mut values = Vec::new();

        while lexer.token != Some(lexer::Token::EOF) {
            lexer.next_token();
            if let Some(tok) = lexer.token {
                let _ = tokens.push(tok);
                let _ = values.push(lexer.value.clone());
            }
        }

        println!("{:?}", tokens);
        println!("{:?}", values);

        println!("");
    }

    // Parser Tests

    // #[test]
    // fn variables_parser_test() {
    //     let input = String::from("a = 5; b = 3;");
    //     let lexer = lexer::Lexer::new(input);
    //
    //     let mut parser = parser::Parser::new(lexer);
    //     let ast = parser.parse();
    //
    //     println!("{:?}", ast);
    // }
}
