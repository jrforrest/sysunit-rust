use std::path::Path;
use std::fs;
use std::io::Read;
use std::process::{Command, Stdio};

use crate::unit::{Instance};
use crate::error::Error;
use crate::fs_util;

use super::{Executor, Operation, Execution, ExecutionResult};

const DEFAULT_DIRS: &'static [&'static str] = &["/usr/lib/sysunit/adapters"];

pub struct Adapter {
    path: String,
}

impl Adapter {
    pub fn try_new(adapter_name: &str) -> Result<Adapter, Error> {
        Ok(Adapter { path: load_executable(adapter_name)? })
    }
}

impl Executor for Adapter {
    fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn execute(&mut self, unit: &Instance, operation: Operation) -> ExecutionResult {
        let definition = unit.definition_rc.clone();
        let unit_path = &definition.path;

        let mut command = Command::new(&self.path);
        command
            .arg(operation.to_str())
            .arg(unit_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = match command.spawn() {
            Ok(c) => c,
            Err(e) => {
                let msg = format!("Could not execute unit adapter `{}`, path: {}, error: {:?}",
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

fn load_executable(adapter_name: &str) -> Result<String, Error> {
    let paths = fs_util::get_path_var("SYSUNIT_ADAPTER_PATH", DEFAULT_DIRS);

    for dir in paths.iter() {
        let dir_path = Path::new(dir);
        let adapter_path = dir_path.join(adapter_name);

        if is_executable(&adapter_path) {
            return Ok(
                adapter_path.to_str()
                .expect("Invalid UTF8 in adapter path")
                .to_string());
        }
    }

    let err_msg = format!("Could not find executable adapter {} in any of {:?}",
        adapter_name,
        paths);

    Err(Error::new(err_msg))
}

fn is_executable(path: &Path) -> bool {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return false
    };

    fs_util::unix::is_executable_file(&metadata)
}
