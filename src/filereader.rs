// hiw-lang compiler
// https://github.com/mealet/hiw-lang
// ----------------------------------------
// Copyright ©️ 2024, mealet.
// Project licensed under the BSD-3 License
// that can be found in LICENSE file.
// ----------------------------------------

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

pub fn search_import(path_to_file: String) -> String {
    // checking if input string is only filename
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let current_dir_file = current_dir.join(&path_to_file);

    let exe_path = std::env::current_exe().expect("Failed to get executable directory");
    let exe_dir = exe_path
        .parent()
        .expect("Faield to get executable directory");
    let exe_path_file = exe_dir.join(&path_to_file);

    match (current_dir_file.exists(), exe_path_file.exists()) {
        (false, false) => return "FILE_NOT_FOUND_1_HIW_ERROR".to_string(),
        (true, false) => return current_dir_file.to_str().unwrap().to_string(),
        // First priority to executable path modules
        (false, true) => return exe_path_file.to_str().unwrap().to_string(),
        (true, true) => return exe_path_file.to_str().unwrap().to_string(),
    }
}
