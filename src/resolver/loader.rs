use std::fs;
use std::path::Path;
use std::env;

use crate::unit::{Definition, DefinitionType};
use crate::error::Error;

const DEFAULT_DIRS: &'static [&'static str] = &["./units", "/etc/units"];

pub fn load_unit(name: &str) -> Result<Definition, Error> {
    let directories = get_directories();

    for dir in directories.iter() {
        let dir_path = Path::new(dir);
        let full_path = dir_path.join(&name);

        match unit_type(&full_path)? {
            None => continue,
            Some(definition_type) => {
                let full_path_os_string = full_path.into_os_string();
                let full_path_str = full_path_os_string.to_str()
                    .expect("Invalid unit path UTF8 string!");
                let unit = Definition::new(name, full_path_str, definition_type);

                return Ok(unit);
            }
        }
    }

    let error = Error::new(format!("Could not find unit `{}` in any of {:?}",
        name,
        directories
    ));

    return Err(error)
}

fn get_directories() -> Vec<String> {
    match env::var("SYSUNIT_PATH") {
        Ok(string) => string.split(":").map(|s| s.to_string()).collect(),
        Err(_) => DEFAULT_DIRS.iter().map(|s| s.to_string()).collect()
    }
}

fn unit_type(full_path: &Path) -> Result<Option<DefinitionType>, Error> {
    let metadata = match fs::metadata(full_path) {
        Ok(m) => m,
        Err(_) => return Ok(None)
    };

    if unix::is_executable_file(&metadata) { return Ok(Some(DefinitionType::Executable))}

    if metadata.is_dir() {
        let unit_executable_path = full_path.join(Path::new("./unit"));
        let unit_executable_meta = match fs::metadata(unit_executable_path) {
            Ok(m) => m,
            Err(_) => {
                let msg = format!("While resolving a unit located in {}, a \
                    directory was found but it does not contain an executable ./unit file.",
                    full_path.to_str().expect("Invalid UTF8 path")
                );
                return Err(Error::new(msg))
            }
        };

        if unix::is_executable_file(&unit_executable_meta) {
            return Ok(Some(DefinitionType::Directory));
        }
    }

    return Ok(None);
}

#[cfg(unix)]
mod unix {
    use std::os::unix::fs::PermissionsExt;
    use std::fs::Metadata;

    pub fn is_executable_file(metadata: &Metadata) -> bool {
        let permissions = metadata.permissions();

        metadata.is_file() && (permissions.mode() & 0o111 != 0)
    }
}
