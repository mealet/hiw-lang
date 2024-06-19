const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn greeting() {
    println!(
        r#"
HIW Compiler | {}
"#,
        APP_VERSION
    )
}
