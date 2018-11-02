use std::process::{Command, Stdio};
mod unit;

use std::io::Write;
use unit::{Definition, Instance, Operation};

struct Adapter<'a> {
    name: &'a str,
    command: &'a str
}

impl<'a> Adapter<'a> {
    pub fn new(name: &'a str, command: &'a str) -> Adapter<'a> {
        Adapter{name: name, command: command}
    }

    pub fn run(&self, instance: &Instance, operation: Operation) {
        let mut cmd = Command::new(self.command)
           .stdout(Stdio::piped())
           .stdin(Stdio::piped())
           .spawn()
           .expect("Failed to start adapter command");

        {
            let script = instance.command_for(operation);
            let stdin = cmd.stdin.as_mut().expect("Failed to open stdin for adapter command");
            stdin.write_all(script.as_bytes()).expect("Failed to write command to adapter");
        }

        let output = cmd.wait_with_output().expect("Failed to read stdout from adapter");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn main() {
    let definition = Definition::new("blarp", "echo hi", "echo bye");
    let adapter = Adapter::new("local", "/bin/sh");

    let instance = definition.get_instance();

    adapter.run(&instance, Operation::Check);
}
