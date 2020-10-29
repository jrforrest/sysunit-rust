use std::process::Command;
use std::process::Stdio;
use std::io::Read;
use std::fs;
use std::path::Path;

use crate::unit::{Instance, DefinitionType};
use crate::error::Error;

use super::super::{Executor, Operation, Execution};

use url::Url;

pub struct Local {
    url: Option<Url>,
}

impl Local {
    pub fn new(url: Option<Url>) -> Local {
        Local { url: url }
    }
}

impl Executor for Local {
    fn init(&mut self) -> Result<(), Error> {
        match self.url {
            Some(ref url) => match url.host() {
                None => Ok(()),
                Some(other) => match other.to_string().as_str() {
                    "localhost" => Ok(()),
                    _ => Err(Error::new(format!("Can not operate on host: {}", other)))
                }
            },
            None => Ok(())
        }
    }

    fn execute(&self, unit: &Instance, operation: Operation) -> Result<Execution, Error> {
        let definition = unit.definition_rc.clone();
        let mut command = match &definition.definition_type {
            DefinitionType::Executable => Command::new(&definition.path),
            DefinitionType::Directory => {
                let path = Path::new(&definition.path);
                let executable_path = path.join(Path::new("unit"));
                let executable_path_str = executable_path.to_str().expect("Could not parse UTF8 path");
                let canon = fs::canonicalize(&executable_path_str).unwrap();
                let mut command = Command::new(&canon.to_str().unwrap());

                command.current_dir(&definition.path);

                command
            }
        };

        let env_iter = unit.id.args.vec.iter().map(|arg|
            (arg.name.clone(), arg.value.clone())
        );

        command
            .arg(operation.to_str())
            .envs(env_iter)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = match command.spawn() {
            Ok(c) => c,
            Err(e) => {
                let msg = format!("Could not execute unit `{}`, path: {}, error: {:?}",
                    definition.name,
                    definition.path,
                    e.kind()
                );

                return Err(Error::new(msg));
            }
        };

        let status = match child.wait() {
            Ok(s) => s,
            Err(_) => {
                let error = Error::new(format!("[{}] killed by external signal", definition.name));
                return Err(error)
            }
        };

        let mut stdout = String::new();
        let mut stderr = String::new();

        child
            .stdout
            .unwrap()
            .read_to_string(&mut stdout)
            .unwrap();

        child
            .stderr
            .unwrap()
            .read_to_string(&mut stderr)
            .unwrap();

        let exit_code = status.code().unwrap();

        let execution = Execution {
            unit_name: definition.name.clone(),
            stdout: stdout,
            stderr: stderr,
            exit_code: exit_code
        };

        Ok(execution)
    }
}
