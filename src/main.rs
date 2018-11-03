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

    pub fn check(&self, instance: &Instance) {
        self.adapter.run(&instance, Operation::Check);
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

struct Executor<'a> {
    instance: &'a Instance<'a>,
    host: &'a Host<'a>
}

impl<'a> Executor<'a> {
    pub fn new(host: &'a Host<'a>, instance: &'a Instance<'a>) -> Executor<'a> {
        Executor{instance: instance, host: host}
    }

    pub fn perform(&self, operation: Operation) {
        for dep in self.instance.iterate_dependencies() {
            let executor = Executor::new(self.host, dep);

            executor.perform(operation);
        }

        match operation {
            Operation::Check => self.host.check(&self.instance),
            Operation::Apply => self.host.apply(&self.instance),
        };
    }
}

fn main() {
    let child_definition = Definition::new("harp", "echo checking child", "false");
    let child_instance = child_definition.get_instance();

    let mut definition = Definition::new("blarp", "echo hi && false", "echo bye");
    definition.depends_on(child_instance);

    let instance = definition.get_instance();

    let host = Host::new();

    let executor = Executor::new(&host, &instance);
    executor.perform(Operation::Apply)
}
