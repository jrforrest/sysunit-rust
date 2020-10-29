use crate::execution::Execution;
use crate::operation::Operation;
use crate::error::Error;

use colored::*;
use unicode_truncate::UnicodeTruncateStr;
use rpassword::read_password_from_tty;

#[derive(Clone, Copy)]
pub enum Mode {
    Full,
    Minimal
}

pub fn report_execution(execution: &Execution, mode: Mode, operation: Operation) {
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
        Mode::Minimal => {
            let first_line = match execution.stdout.lines().into_iter().nth(0) {
                Some(s) => s,
                None => ""
            };
            format!(" {}", first_line.unicode_truncate(40).0.trim())
        }
    };

    println!("[{}|{}]{}", unit_name_colored, operation.to_str(), output_reporting)
}

pub fn prefix_lines(output: &str, prefix: &str) -> String {
    let mut output_string = String::new();

    for line in output.lines() {
        output_string.push_str(&format!("{}{}", prefix, line));
    }
    
    output_string
}

pub fn prompt_ssh_password(username: &str, host: &str) -> Result<String, Error> {
    let prompt = format!("SSH Password {}@{}: ", username, host);

    read_password_from_tty(Some(prompt.as_str()))
        .map_err(|e| Error::new(format!("{}", e)))
}
