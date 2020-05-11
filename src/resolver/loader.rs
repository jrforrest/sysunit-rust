use std::fs;
use std::path::Path;

use crate::unit::{Definition, DefinitionType};
use crate::error::Error;
use crate::fs_util;

const DEFAULT_DIRS: &'static [&'static str] = &["./units", "/etc/units"];

pub fn load_unit(name: &str) -> Result<Definition, Error> {
    let directories = fs_util::get_path_var("SYSUNIT_PATH", DEFAULT_DIRS);

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

fn unit_type(full_path: &Path) -> Result<Option<DefinitionType>, Error> {
    let metadata = match fs::metadata(full_path) {
        Ok(m) => m,
        Err(_) => return Ok(None)
    };

    if fs_util::unix::is_executable_file(&metadata) {
        return Ok(Some(DefinitionType::Executable))
    }

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

        if fs_util::unix::is_executable_file(&unit_executable_meta) {
            return Ok(Some(DefinitionType::Directory));
        }
    }

    return Ok(None);
}
