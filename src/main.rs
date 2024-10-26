use std::env;

const ARG_PREFIX: &str = "-";

enum Mode {
    Default,
    WithNumbering { igonre_empty_lines: bool }
}

fn main() {
    let (options, files) = get_input();
    if files.is_empty() {
        show_help();
        return;
    }
}

fn get_input() -> (Vec<String>, Vec<String>) {
    env::args().skip(1).partition(|arg| arg.starts_with(ARG_PREFIX))
}

fn show_help() {
    println!("Usage:");
    println!("cat [option] file1, file2 ...");
    println!("option:");
    println!("  -n - show line numbers");
    println!("  -nb - show line numbers, ignore blank lines");
}