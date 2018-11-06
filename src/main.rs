#[macro_use]
extern crate serde_derive;
extern crate glob;
extern crate serde_yaml;

use std::process::{Command, Stdio};
mod unit;
mod loader;

use std::io::Write;
use unit::{Instance, Operation};
use loader::Loader;

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

    pub fn check(&self, instance: &Instance) -> Result<(), ()> {
       self.adapter.run(&instance, Operation::Check)
    }

    pub fn apply(&self, instance: &Instance) -> Result<(), ()> {
        match self.adapter.run(&instance, Operation::Check) {
            Err(()) => {
                self.adapter.run(&instance, Operation::Apply)
            }
            Ok(()) => Ok(()),
        }
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

    pub fn perform(&self, operation: Operation) -> Result<(), ()>{
        for dep in self.instance.iterate_dependencies() {
            let executor = Executor::new(self.host, dep);

            executor.perform(operation)?;
        }

        match operation {
            Operation::Check => self.host.check(&self.instance),
            Operation::Apply => self.host.apply(&self.instance),
        }
    }
}

fn main() {
    let loader = Loader::new();
    loader.load("./units");

    let definition = loader.find("yup");
    let instance = definition.get_instance();

    let host = Host::new();
    let executor = Executor::new(&host, &instance);

    executor.perform(Operation::Apply).expect("failed to apply");
}
