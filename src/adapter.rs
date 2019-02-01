use std::process::{Command, Stdio};
use std::io::Write;
use unit::{Instance, Operation};

pub struct Adapter<'a> {
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

