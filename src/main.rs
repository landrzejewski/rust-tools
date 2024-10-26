use std::{env, fs::File, io::{BufRead, BufReader}};

const ARG_PREFIX: &str = "-";

enum Mode {
    Default,
    WithNumbering { igonre_empty_lines: bool }
}

impl Mode {
    
    fn from(options: &Vec<String>) -> Mode {
        if options.is_empty() {
            Mode::Default
        } else {
            match options[0].as_str() {
                "-n" => Mode::WithNumbering { igonre_empty_lines: false },
                "-nb" => Mode::WithNumbering { igonre_empty_lines: true },
                _ => Mode::Default
            }
        }
    }

}

fn main() {
    let (options, paths) = get_input();
    if paths.is_empty() {
        show_help();
        return;
    }
    let mode = Mode::from(&options);
    cat(&mode, &paths);
}

fn get_input() -> (Vec<String>, Vec<String>) {
    env::args().skip(1).partition(|arg| arg.starts_with(ARG_PREFIX))
}

fn show_help() {
    println!("Usage:");
    println!("cat [option] file1, file2 ...");
    println!("options:");
    println!("  -n - show line numbers");
    println!("  -nb - show line numbers, ignore blank lines");
}

fn cat(mode: &Mode, paths: &Vec<String>) {
    let print_lines = match mode {
        Mode::WithNumbering { igonre_empty_lines: false } => print_with_line_numbers,
        Mode::WithNumbering { igonre_empty_lines: true } => print_with_line_numbers_ignoring_empty_lines,
        _ => print_line
    };
    for path in paths {
        let Ok(file) = File::open(path) else {
            println!("Failed to open {path}");
            continue;
        };
        println!("File: {path}");
        let mut line_number = 0;
        let reader = BufReader::new(file);  
        for line in reader.lines() {
            let line_text = line.expect("Failed to read line");
            print_lines(&mut line_number, &line_text);
        }
    }
}

fn print_line(_line_number: &mut i32, line: &String) {
    println!("{line}");
}

fn print_with_line_numbers(line_number: &mut i32, line: &String) {
    *line_number += 1;
    println!("{:3}:\t{}", line_number, line);
}

fn print_with_line_numbers_ignoring_empty_lines(line_number: &mut i32, line: &String) {
    if line.is_empty() {
        println!();
    } else {
        print_with_line_numbers(line_number, line);
    }
}
