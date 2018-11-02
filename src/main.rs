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

    pub fn run(&self, instance: &Instance, operation: Operation) -> Result<(), ()> {
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

        match output.status.success() {
            true => Ok(()),
            false => Err(())
        }
    }
}

struct Host<'a> {
    adapter: Adapter<'a>,
}

impl<'a> Host<'a> {
    pub fn new() -> Host<'a> {
        let adapter = Adapter::new("local", "/bin/sh");
        Host{adapter: adapter}
    }

    pub fn apply(&self, instance: &Instance) {
        match self.adapter.run(&instance, Operation::Check) {
            Err(()) => {
                self.adapter.run(&instance, Operation::Apply);
            }
            Ok(()) => (),
        };
    }
}

fn main() {
    let definition = Definition::new("blarp", "echo hi && false", "echo bye");
    let host = Host::new();

    let instance = definition.get_instance();

    host.apply(&instance)
}
