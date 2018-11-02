use std::process::{Command, Stdio};
use std::io::Write;

struct UnitDefinition<'a> {
    name: &'a str,
    check: &'a str
}

impl<'a> UnitDefinition<'a> {
    pub fn new(name: &'a str, check: &'a str) -> UnitDefinition<'a> {
        UnitDefinition{name: name, check: check}
    }
}

struct Adapter<'a> {
    name: &'a str,
    command: &'a str
}

impl<'a> Adapter<'a> {
    pub fn new(name: &'a str, command: &'a str) -> Adapter<'a> {
        Adapter{name: name, command: command}
    }

    pub fn run(&self, definition: &UnitDefinition) {
        let mut cmd = Command::new(self.command)
           .stdout(Stdio::piped())
           .stdin(Stdio::piped())
           .spawn()
           .expect("Failed to start adapter command");

        {
            let mut stdin = cmd.stdin.as_mut().expect("Failed to open stdin for adapter command");
            stdin.write_all(definition.check.as_bytes()).expect("Failed to write command to adapter");
        }

        let output = cmd.wait_with_output().expect("Failed to read stdout from adapter");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn main() {
    let definition = UnitDefinition::new("blarp", "echo hi");
    let adapter = Adapter::new("local", "/bin/sh");

    adapter.run(&definition);
}
