// Filereader - simple module for reading and formatting code

use std::fs;

fn error(message: String) {
    eprintln!("{}", message);
    std::process::exit(1);
}

pub fn get_code(path_to_file: String) -> String {
    // reading code from source
    let source_code = match fs::read_to_string(&path_to_file) {
        Ok(f) => f,
        Err(e) => {
            error(format!("{} | Error while opening:\n{}", &path_to_file, e));
            "\0".to_string()
        }
    };

    // formatting code
    let formatted_code = source_code.replace("\n", "").replace("\r", "");

    formatted_code
}
