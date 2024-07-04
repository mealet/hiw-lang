// Filereader - simple module for reading and formatting code

use std::fs;

fn error(message: String) {
    eprintln!("{}", message);
    std::process::exit(1);
}

pub fn remove_comments(text: String) -> String {
    text.lines()
        .map(|line| {
            if let Some(index) = line.find("//") {
                &line[..index]
            } else {
                line
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
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

    let formatted_code = remove_comments(source_code)
        // .replace("\n", "")
        .replace("\r", "");

    formatted_code
}
