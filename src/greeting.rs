const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_NAME: &str = env!("CARGO_PKG_NAME");

use colored::Colorize;

pub fn greeting() {
    println!(
        "{}",
        format!("{}", format!("| {} | {}", APP_NAME, APP_VERSION).cyan())
    )
}
