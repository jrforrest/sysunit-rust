use crate::execution::Execution;
use colored::*;
use unicode_truncate::UnicodeTruncateStr;

#[derive(Clone, Copy)]
pub enum Mode {
    Full,
    Minimal
}

pub fn report_execution(execution: &Execution, mode: Mode) {
    let unit_name_colored = if execution.success() {
        execution.unit_name.green()
    } else {
        execution.unit_name.red()
    };

    let output_reporting = match mode {
        Mode::Full => 
            format!("\n{}\n{}",
                prefix_lines(&execution.stdout, "1>"),
                prefix_lines(&execution.stderr, "2>")),
        Mode::Minimal =>
            format!(" {}", execution.stdout.unicode_truncate(40).0.trim()),
    };

    println!("[{}]{}", unit_name_colored, output_reporting)
}
pub fn prefix_lines(output: &str, prefix: &str) -> String {
    let mut output_string = String::new();

    for line in output.lines() {
        output_string.push_str(&format!("{}{}", prefix, line));
    }
    
    output_string
}
