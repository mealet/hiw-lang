// hiw-lang compiler
// https://github.com/mealet/hiw-lang
// ----------------------------------------
// Copyright ©️ 2024, mealet.
// Project licensed under the BSD-3 License
// that can be found in LICENSE file.
// ----------------------------------------

// Greeting - simple module for greeting user

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_NAME: &str = env!("CARGO_PKG_NAME");

use colored::Colorize;

pub fn greeting() {
    println!(
        "{}",
        format!("{}", format!("| {} | {}", APP_NAME, APP_VERSION).cyan())
    )
}
